use add::AddFunc;
use divide::DivideFunc;
use inverse::InverseFunc;
use modulus::ModulusFunc;
use multiply::MultiplyFunc;
use negate::NegateFunc;
use power::PowerFunc;
use square::SquareFunc;
use square_root::SquareRootFunc;
use subtract::SubtractFunc;

use crate::state::RpnState;

pub mod add;
pub mod divide;
pub mod inverse;
pub mod modulus;
pub mod multiply;
pub mod negate;
pub mod power;
pub mod square;
pub mod square_root;
pub mod subtract;

pub fn basic_register_functions(state: &mut RpnState) {
    state.register_function(Box::new(AddFunc::new()));
    state.register_function(Box::new(DivideFunc::new()));
    state.register_function(Box::new(InverseFunc::new()));
    state.register_function(Box::new(ModulusFunc::new()));
    state.register_function(Box::new(MultiplyFunc::new()));
    state.register_function(Box::new(NegateFunc::new()));
    state.register_function(Box::new(PowerFunc::new()));
    state.register_function(Box::new(SquareRootFunc::new()));
    state.register_function(Box::new(SquareFunc::new()));
    state.register_function(Box::new(SubtractFunc::new()));
}
