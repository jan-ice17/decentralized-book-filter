use ic_cdk::api::call::call;
use ic_cdk_macros::{update, query};
use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::collections::HashSet;
use regex::Regex;

thread_local! {
    // Predefined list of common filtered words, plus user-added words.
    static FILTERED_WORDS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Add a custom filtered word.
#[update]
fn add_filtered_word(word: String) -> String {
    let sanitized_word = word.trim().to_lowercase(); // Sanitize input
    FILTERED_WORDS.with(|filtered_words| {
        let mut words = filtered_words.borrow_mut();
        if words.insert(sanitized_word) {
            "Custom filtered word added.".to_string()
        } else {
            "Filtered word already exists.".to_string()
        }
    })
}

/// Function to fetch the uploaded book content from `ContentUploader`.
#[update]
async fn fetch_uploaded_book() -> String {
    let content_uploader_id = Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").expect("Invalid ContentUploader canister ID");
    match call::<(), (String,)>(content_uploader_id, "get_ebook", ()).await {
        Ok((content,)) => content,
        Err(_) => "Error: Could not retrieve content.".to_string(),
    }
}

/// Filter out filtered words from the fetched content.
#[update]
async fn filter_content() -> String {
    let original_content = fetch_uploaded_book().await;

    if original_content.contains("Error:") || original_content.is_empty() {
        return original_content; // Return early if there's an error
    }

    let filtered_words = FILTERED_WORDS.with(|filtered_words| {
        filtered_words.borrow().clone()
    });

    // Use regex for efficient filtering
    let regex_pattern = filtered_words.iter().map(|w| regex::escape(w)).collect::<Vec<String>>().join("|");
    let re = Regex::new(&regex_pattern).unwrap();
    let filtered_content = re.replace_all(&original_content, |caps: &regex::Captures| {
        "*".repeat(caps.get(0).unwrap().as_str().len())
    });

    filtered_content.to_string()
}

/// Retrieve the current list of filtered words.
#[query]
fn get_filtered_words() -> Vec<String> {
    FILTERED_WORDS.with(|filtered_words| filtered_words.borrow().iter().cloned().collect())
}
