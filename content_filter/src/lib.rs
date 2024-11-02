use ic_cdk::api::call::call;
use ic_cdk_macros::{update, query};
use ic_cdk::export::Principal;
use std::cell::RefCell;

thread_local! {
    // Predefined list of common filtered words, plus user-added words.
    static FILTERED_WORDS: RefCell<Vec<String>> = RefCell::new(vec![
        "fool".to_string(),
        "damn".to_string(),
        "bitch".to_string(),
        "fucked".to_string(),
        "idiot".to_string(),
        "shit".to_string(),
        "crap".to_string(),
        "stupid".to_string(),
        "dumb".to_string(),
        "jerk".to_string(),
    ]);
}

/// Add a custom filtered word.
#[update]
fn add_filtered_word(word: String) -> String {
    FILTERED_WORDS.with(|filtered_words| {
        filtered_words.borrow_mut().push(word);
    });
    "Custom filtered word added.".to_string()
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

    if original_content == "Error: Could not retrieve content." || original_content == "No content uploaded." {
        return original_content;
    }

    let filtered_content = FILTERED_WORDS.with(|filtered_words| {
        let filtered_words = filtered_words.borrow();
        let mut content = original_content.clone();

        for filtered_word in filtered_words.iter() {
            let replacement = "*".repeat(filtered_word.len());
            content = content.replace(filtered_word, &replacement);
        }
        content
    });

    filtered_content
}

/// Retrieve the current list of filtered words.
#[query]
fn get_filtered_words() -> Vec<String> {
    FILTERED_WORDS.with(|filtered_words| filtered_words.borrow().clone())
}
