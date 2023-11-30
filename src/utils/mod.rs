mod clipboard;

pub use clipboard::Clipboard;

#[cfg(feature = "copypasta")]
pub use clipboard::copypasta::CopypastaClipboard;

#[cfg(not(feature = "copypasta"))]
pub use clipboard::noop::NoopClipboard;

#[cfg(test)]
pub use clipboard::test::MockClipboard;
