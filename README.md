Here’s the documentation with the limitations section added concisely:

---

# 📚 Decentralized Book Filter: A Safer Way to Read

**Author:** Janice Gathoga

## Project Overview

Welcome to the **Decentralized Book Filter**! 🌐📖 This platform allows users to upload eBooks and filter out inappropriate words, creating a safer reading experience. Built on the **Internet Computer (ICP)**, it ensures secure and scalable storage on a decentralized infrastructure.

---

## Why Use This Platform? 🤔

Imagine you’re a teacher or parent wanting to share a book with younger readers. 📖 Some books may have language that’s not suitable for them. **Decentralized Book Filter** automatically removes or blurs these words, ensuring a safe reading environment for all ages, free from centralized control.

---

## Project Structure 🏗️

1. **ContentUploader** 📤: Manages eBook uploads.
2. **ContentFilter** 🧹: Filters words, allowing users to add their own.
3. **ContentManager** 🛠️: Sets filter modes (blur/remove) and retrieves filtered content.

---

## Getting Started: Installation & Setup 🛠️

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/jan-ice17/decentralized-book-filter.git
   cd decentralized-book-filter/ebook_filter_canisters
   ```

2. **Start the Internet Computer**:  
   ```bash
   dfx start --background
   ```

3. **Deploy the Canisters**:  
   ```bash
   dfx deploy
   ```

---

## How to Use the Book Filter System ✨

**Example Scenario:** Upload the sentence: `"This guy is such an idiot! What a damn mess."`

### Step-by-Step

1. **Upload Content**:
   ```bash
   dfx canister call content_uploader upload_ebook '( "This guy is such an idiot! What a damn mess." )'
   ```
   
2. **Verify Content**:
   ```bash
   dfx canister call content_uploader get_ebook
   ```
   
3. **Choose Filter Mode**:
   - **Blur Mode**:
     ```bash
     dfx canister call content_manager set_filter_mode '( "blur" )'
     ```
   - **Remove Mode**:
     ```bash
     dfx canister call content_manager set_filter_mode '( "remove" )'
     ```
   
4. **View Filtered Content**:
   ```bash
   dfx canister call content_manager fetch_and_filter_content
   ```

---

## Current Limitations ⚠️

1. **File Size**: Limited to small text files currently; full books require stable memory.
2. **Basic Filtering**: Filters only exact matches, so variations of words (e.g., "foolish" for "fool") won’t be filtered.
3. **Language Support**: Currently focused on English.
4. **Permissions**: No role-based access control yet.
5. **No Context Analysis**: The filter may replace words that are contextually appropriate (e.g., “dam” in a river context).

---

## Summary 📖

The **Decentralized Book Filter** by **Janice Gathoga** provides a customizable, decentralized solution for safer reading. It’s ideal for educators, parents, and anyone wanting control over digital reading content. Future updates aim to expand features and address limitations for an even more robust platform.

Happy filtering! 🎉📚