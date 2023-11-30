mod clipboard;

pub use clipboard::{Clipboard, CopypastaClipboard};

#[cfg(test)]
pub use clipboard::test::MockClipboard;
