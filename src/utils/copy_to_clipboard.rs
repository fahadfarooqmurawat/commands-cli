use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(text: String) -> () {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    if let Err(_e) = ctx.set_contents(text) {
        println!("Failed to copy to clipboard");
    } else {
        println!("Copied to clipboard");
    }
}
