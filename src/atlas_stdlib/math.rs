use crate::{
    atlas_memory::vm_data::VMData,
    atlas_vm::{errors::RuntimeError, CallBack},
};
use rand::{thread_rng, Rng};

use crate::atlas_vm::vm_state::VMState;

pub const MATH_ATLAS: &str = include_str!("math.atlas");

pub const MATH_FUNCTIONS: [(&str, CallBack); 7] = [
    ("abs", abs),
    ("pow", pow),
    ("sqrt", sqrt),
    ("min", min),
    ("max", max),
    ("round", round),
    ("random", random),
];

pub fn abs(state: VMState) -> Result<VMData, RuntimeError> {
    let val = state.stack.pop()?.as_i64();
    Ok(VMData::new_i64(val.abs()))
}

pub fn pow(state: VMState) -> Result<VMData, RuntimeError> {
    let exponent = state.stack.pop()?.as_i64();
    let base = state.stack.pop()?.as_i64();
    Ok(VMData::new_i64(base.pow(exponent as u32)))
}

pub fn sqrt(state: VMState) -> Result<VMData, RuntimeError> {
    let val = state.stack.pop()?.as_f64();
    Ok(VMData::new_f64(val.sqrt()))
}

pub fn min(state: VMState) -> Result<VMData, RuntimeError> {
    let v1 = state.stack.pop()?.as_i64();
    let v2 = state.stack.pop()?.as_i64();
    Ok(VMData::new_i64(std::cmp::min(v1, v2)))
}

pub fn max(state: VMState) -> Result<VMData, RuntimeError> {
    let v1 = state.stack.pop()?.as_i64();
    let v2 = state.stack.pop()?.as_i64();
    Ok(VMData::new_i64(std::cmp::max(v1, v2)))
}

pub fn round(state: VMState) -> Result<VMData, RuntimeError> {
    let val = state.stack.pop()?.as_f64();
    Ok(VMData::new_i64(val.round() as i64))
}

pub fn random(state: VMState) -> Result<VMData, RuntimeError> {
    let range = (state.stack.pop()?.as_i64(), state.stack.pop()?.as_i64());
    let mut rng = thread_rng();
    let random = rng.gen_range(range.1..range.0);
    Ok(VMData::new_i64(random))
}
