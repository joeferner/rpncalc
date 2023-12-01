use crate::error::RpnCalcError;
use crate::functions::function::Function;
use crate::functions::Category;
use crate::rpn_calc::RpnCalc;
use crate::stack_item::StackItem;
use std::fmt::{Display, Formatter};

pub struct Sin {}

impl Sin {
    pub fn new() -> Self {
        return Sin {};
    }
}

impl Display for Sin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "sin")
    }
}

impl Function for Sin {
    fn apply(&self, rpn_calc: &mut RpnCalc) -> Result<(), RpnCalcError> {
        return rpn_calc.execute_unary_number_operator(|rpn_calc, a| {
            let result = a.sin(rpn_calc.angle_mode)?;
            rpn_calc.push(StackItem::Number(result));
            return Ok(());
        });
    }

    fn get_help(&self) -> String {
        return "Calculates the sine of the top item on the stack using the current angle mode".to_string();
    }
    fn get_category(&self) -> Category {
        return Category::Trig;
    }
}

#[cfg(test)]
mod tests {
    use crate::number::Number;
    use crate::rpn_calc::tests::{run_unary_operator_deg, run_unary_operator_grad, run_unary_operator_rad};

    #[test]
    fn test_sin_deg() {
        run_unary_operator_deg("10", "sin", Number::from(0.17364817766693033));
    }

    #[test]
    fn test_sin_rad() {
        run_unary_operator_rad("0.34", "sin", Number::from(0.3334870921408144));
    }

    #[test]
    fn test_sin_grad() {
        run_unary_operator_grad("0.34", "sin", Number::from(0.00534068212216596877792528991418));
    }

    #[test]
    fn test_sin_units_deg() {
        run_unary_operator_rad("10 deg", "sin", Number::from(0.17364817766693033));
    }

    #[test]
    fn test_sin_units_rad() {
        run_unary_operator_deg("0.34 rad", "sin", Number::from(0.3334870921408144));
    }

    #[test]
    fn test_sin_units_grad() {
        run_unary_operator_deg("0.34 grad", "sin", Number::from(0.00534068212216596877792528991418));
    }
}
