use ic_cdk::api::call::call;
use ic_cdk_macros::{update, query};
use ic_cdk::export::Principal;
use std::cell::RefCell;

thread_local! {
    static FILTER_MODE: RefCell<String> = RefCell::new("blur".to_string());
}

#[update]
fn set_filter_mode(mode: String) -> String {
    FILTER_MODE.with(|m| {
        *m.borrow_mut() = mode.clone();
    });
    format!("Filter mode set to '{}'.", mode)
}

#[query]
fn get_filter_mode() -> String {
    FILTER_MODE.with(|m| m.borrow().clone())
}

#[update]
async fn fetch_and_filter_content() -> String {
    let content_uploader_id = Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap();
    let content_filter_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

    // Fetch the content from the ContentUploader canister
    let result: Result<(String,), _> = call(content_uploader_id, "get_ebook", ()).await;
    let content = match result {
        Ok((text,)) => text,
        Err(_) => return "Error: Could not retrieve content from ContentUploader.".to_string(),
    };

    // Get the list of filtered words from the ContentFilter canister
    let filtered_words_result: Result<(Vec<String>,), _> = call(content_filter_id, "get_filtered_words", ()).await;
    let filtered_words = match filtered_words_result {
        Ok((words,)) => words,
        Err(_) => return "Error: Could not retrieve filtered words.".to_string(),
    };

    // Get the filter mode (e.g., "blur" or "remove")
    let mode = get_filter_mode();

    // Apply the filter based on the mode
    let filtered_content = if mode == "remove" {
        // For "remove" mode, replace each filtered word with an empty string
        let mut filtered_text = content.clone();
        for word in &filtered_words {
            filtered_text = filtered_text.replace(word, "");
        }
        filtered_text
    } else {
        // Default to "blur" mode, replacing each filtered word with ***
        let mut filtered_text = content.clone();
        for word in &filtered_words {
            let replacement = "*".repeat(word.len());
            filtered_text = filtered_text.replace(word, &replacement);
        }
        filtered_text
    };

    filtered_content
}
