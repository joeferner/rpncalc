use crate::error::RpnCalcError;

pub trait Clipboard {
    fn set_contents(&mut self, data: String) -> Result<(), RpnCalcError>;

    fn get_contents(&mut self) -> Result<Option<String>, RpnCalcError>;
}

#[cfg(feature = "copypasta")]
pub mod copypasta {
    use crate::error::RpnCalcError;
    use crate::utils::Clipboard;
    use copypasta::{ClipboardContext, ClipboardProvider};

    pub struct CopypastaClipboard {
        ctx: ClipboardContext,
    }

    impl CopypastaClipboard {
        pub fn new() -> Result<Self, RpnCalcError> {
            let ctx = ClipboardContext::new().map_err(|err| RpnCalcError::GenericError(format!("{}", err)))?;
            return Ok(CopypastaClipboard { ctx });
        }
    }

    impl Clipboard for CopypastaClipboard {
        fn set_contents(&mut self, data: String) -> Result<(), RpnCalcError> {
            self.ctx
                .set_contents(data)
                .map_err(|err| RpnCalcError::GenericError(format!("{}", err)))?;
            return Ok(());
        }

        fn get_contents(&mut self) -> Result<Option<String>, RpnCalcError> {
            let contents = self
                .ctx
                .get_contents()
                .map_err(|err| RpnCalcError::GenericError(format!("{}", err)))?;
            return Ok(Some(contents));
        }
    }
}

#[cfg(not(feature = "copypasta"))]
pub mod noop {
    use crate::error::RpnCalcError;
    use crate::utils::Clipboard;

    pub struct NoopClipboard {}

    impl NoopClipboard {
        pub fn new() -> Self {
            return NoopClipboard {};
        }
    }

    impl Clipboard for NoopClipboard {
        fn set_contents(&mut self, _data: String) -> Result<(), RpnCalcError> {
            return Ok(());
        }

        fn get_contents(&mut self) -> Result<Option<String>, RpnCalcError> {
            return Ok(Some("".to_string()));
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::error::RpnCalcError;
    use crate::utils::Clipboard;
    use std::cell::RefCell;
    use std::rc::Rc;

    thread_local! {
        static CLIPBOARD: Rc<RefCell<MockClipboard>> =  Rc::new(RefCell::new(MockClipboard {
            contents: None
        }));
    }

    pub struct MockClipboard {
        contents: Option<String>,
    }

    impl MockClipboard {
        pub fn get() -> Rc<RefCell<MockClipboard>> {
            return CLIPBOARD.with(|c| c.clone());
        }
    }

    impl Clipboard for MockClipboard {
        fn set_contents(&mut self, data: String) -> Result<(), RpnCalcError> {
            self.contents = Some(data);
            return Ok(());
        }

        fn get_contents(&mut self) -> Result<Option<String>, RpnCalcError> {
            return Ok(self.contents.clone());
        }
    }
}
