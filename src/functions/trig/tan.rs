use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Tan {}

impl Tan {
    pub fn new() -> Self {
        return Tan {};
    }
}

impl Display for Tan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tan")
    }
}

impl Function for Tan {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_unary_number_operator(|rpn_calc, a| {
            let result = a.tan(rpn_calc.angle_mode)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Calculates the tangent of the top item on the stack using the current angle mode".to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Trig;
    }
}

#[cfg(test)]
mod tests {
    use crate::number::Number;
    use crate::rpn_calc::tests::run_unary_operator_deg;

    #[test]
    fn test_tan() {
        run_unary_operator_deg("10", "tan", Number::from(0.17632698070846498));
    }
}
