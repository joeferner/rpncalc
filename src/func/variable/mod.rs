use store::StoreFunc;

use crate::state::RpnState;

pub mod store;

pub fn variable_register_functions(state: &mut RpnState) {
    state.register_function(Box::new(StoreFunc::new()));
}
