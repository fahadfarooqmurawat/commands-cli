use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(text: String) -> () {
    let ctx: Result<ClipboardContext, Box<dyn std::error::Error>> = ClipboardProvider::new();

    match ctx {
        Ok(mut clipboard) => {
            if let Err(_e) = clipboard.set_contents(text) {
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
