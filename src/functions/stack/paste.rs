use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::utils::Clipboard;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct Paste {
    clipboard: Rc<RefCell<dyn Clipboard>>,
}

impl Paste {
    pub fn new(clipboard: Rc<RefCell<dyn Clipboard>>) -> Self {
        return Paste { clipboard };
    }
}

impl Display for Paste {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "paste")
    }
}

impl Function for Paste {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return if let Some(contents) = self.clipboard.borrow_mut().get_contents()? {
            rpn_calc.push_str(contents.as_str())?;
            Ok(())
        } else {
            Err(RpnCalcError::InvalidArgument(
                "No or invalid data on the clipboard".to_string(),
            ))
        };
    }

    fn get_help(&self) -> String {
        return "Paste the clipboard text onto the stack.".to_string();
    }

    fn get_category(&self) -> Category {
        return Category::Stack;
    }
}

#[cfg(test)]
mod tests {
    use crate::error::RpnCalcError;
    use crate::rpn_calc::tests::{assert_stack, run};
    use crate::utils::Clipboard;
    use crate::utils::MockClipboard;

    #[test]
    fn test_paste() {
        {
            let clipboard = MockClipboard::get();
            let mut clipboard = clipboard.borrow_mut();
            clipboard.set_contents("1.234".to_string()).unwrap();
        }

        let rpn_calc = run(vec!["10", "paste"]);
        assert_stack(&rpn_calc, vec!["10", "1.234"]);
    }

    #[test]
    fn test_paste_base_value() {
        {
            let clipboard = MockClipboard::get();
            let mut clipboard = clipboard.borrow_mut();
            clipboard.set_contents("zzzzzzzz".to_string()).unwrap();
        }

        let mut rpn_calc = run(vec!["10"]);
        assert_eq!(
            rpn_calc.push_str("paste"),
            Err(RpnCalcError::ParseStackItem("zzzzzzzz".to_string()))
        );
    }
}
