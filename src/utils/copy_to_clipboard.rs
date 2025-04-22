use arboard::Clipboard;

pub fn copy_to_clipboard(text: String) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(_e) = clipboard.set_text(text) {
                println!("Failed to copy to clipboard");
            } else {
                println!("Copied to clipboard");
            }
        }
        Err(_) => {
            println!("Clipboard functionality not available on this machine");
        }
    }
}
