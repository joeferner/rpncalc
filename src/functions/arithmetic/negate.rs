use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Negate {}

impl Negate {
    pub fn new() -> Self {
        return Negate {};
    }
}

impl Display for Negate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "neg")
    }
}

impl Function for Negate {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_unary_number_operator(|rpn_calc, a| {
            let result = a.negate()?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Negate the first item on the stack.".to_string();
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
    fn test_negate() {
        run_unary_operator("10.24", "neg", Number::from(-10.24));
    }
}
