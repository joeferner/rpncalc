use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::number::Number;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct SquareRoot {}

impl SquareRoot {
    pub fn new() -> Self {
        return SquareRoot {};
    }
}

impl Display for SquareRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pow")
    }
}

impl Function for SquareRoot {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_unary_number_operator(|rpn_calc, a| {
            let result = a.pow(&Number::from(0.5))?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Take the square root of the top item on the stack".to_string();
    }

    fn get_category(&self) -> Category {
        return Category::Arithmetic;
    }
}

#[cfg(test)]
mod tests {
    use crate::number::Number;
    use crate::rpn_calc::tests::run_unary_operator;

    #[test]
    fn test_sqrt() {
        run_unary_operator("10.24", "sqrt", Number::from(3.2));
    }
}
