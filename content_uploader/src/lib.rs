use ic_cdk_macros::{update, query};
use std::cell::RefCell;

thread_local! {
    // Thread-local storage to hold the eBook content.
    static CONTENT: RefCell<Option<String>> = RefCell::new(None);
}

/// Function to upload eBook content.
/// This function takes a `String` containing the eBook content and stores it.
/// Returns a success message once the content is stored.
#[update]
fn upload_ebook(content: String) -> String {
    CONTENT.with(|c| {
        *c.borrow_mut() = Some(content);
    });
    "eBook content uploaded successfully.".to_string()
}

/// Function to retrieve the uploaded eBook content.
/// Returns the content if it exists or an error message if nothing has been uploaded.
#[query]
fn get_ebook() -> String {
    CONTENT.with(|c| {
        match &*c.borrow() {
            Some(content) => content.clone(),
            None => "No content uploaded.".to_string(),
        }
    })
}
