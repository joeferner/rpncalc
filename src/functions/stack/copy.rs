use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::utils::Clipboard;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct Copy {
    clipboard: Rc<RefCell<dyn Clipboard>>,
}

impl Copy {
    pub fn new(clipboard: Rc<RefCell<dyn Clipboard>>) -> Self {
        return Copy { clipboard };
    }
}

impl Display for Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "copy")
    }
}

impl Function for Copy {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        let stack_item = rpn_calc.peek();
        return match stack_item {
            Some(si) => {
                let s = rpn_calc.format_stack_item(&si);
                self.clipboard.borrow_mut().set_contents(s)?;
                Ok(())
            }
            None => Err(RpnCalcError::NotEnoughArguments),
        };
    }

    fn get_help(&self) -> String {
        return "Copy the top item on the stack to the clipboard.".to_string();
    }

    fn get_category(&self) -> Category {
        return Category::Stack;
    }
}

#[cfg(test)]
mod tests {
    use crate::rpn_calc::tests::{assert_stack, run};
    use crate::utils::Clipboard;
    use crate::utils::MockClipboard;

    #[test]
    fn test_copy() {
        let rpn_calc = run(vec!["10", "copy"]);
        assert_stack(&rpn_calc, vec!["10"]);
        {
            let clipboard = MockClipboard::get();
            let mut clipboard = clipboard.borrow_mut();
            assert_eq!("10", clipboard.get_contents().unwrap().unwrap());
        }
    }
}
