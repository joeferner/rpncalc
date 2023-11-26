use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Subtract {}

impl Subtract {
    pub fn new() -> Self {
        return Subtract {};
    }
}

impl Display for Subtract {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "sub")
    }
}

impl Function for Subtract {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_binary_number_operator(|rpn_calc, a, b| {
            let result = a.subtract(&b)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Subtract the top two items on the stack".to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Arithmetic;
    }
}

#[cfg(test)]
mod tests {
    use crate::number::Number;
    use crate::rpn_calc::tests::run_binary_operator;

    #[test]
    fn test_subtract() {
        run_binary_operator("1.2", "0.8", "-", Number::from(0.4));
    }
}
