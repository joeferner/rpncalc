use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use std::fmt::{Display, Formatter};

pub struct Duplicate {}

impl Duplicate {
    pub fn new() -> Self {
        return Duplicate {};
    }
}

impl Display for Duplicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "dup")
    }
}

impl Function for Duplicate {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        if let Some(si) = rpn_calc.stack.items.last() {
            rpn_calc.push(si.clone());
        } else {
            return Err(RpnCalcError::NotEnoughArguments);
        }
        return Ok(());
    }

    fn get_help(&self) -> String {
        return "Duplicate the top item on the stack.".to_string();
    }

    fn get_category(&self) -> Category {
        return Category::Stack;
    }
}

#[cfg(test)]
mod tests {
    use crate::rpn_calc::tests::{assert_stack, run};

    #[test]
    fn test_duplicate() {
        let rpn_calc = run(vec!["1", "10", "dup"]);
        assert_stack(&rpn_calc, vec!["1", "10", "10"]);
    }
}
