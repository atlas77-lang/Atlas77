use crate::atlas_vm::errors::RuntimeError;
use crate::atlas_vm::memory::vm_data::VMData;
use crate::atlas_vm::runtime::vm_state::VMState;
use crate::atlas_vm::CallBack;
use rand::{rng, Rng};

pub const MATH_FUNCTIONS: [(&str, CallBack); 7] = [
    ("abs", abs),
    ("pow", pow),
    ("sqrt", sqrt),
    ("min", min),
    ("max", max),
    ("round", round),
    ("random", random),
];

pub fn abs<'lib>(state: VMState) -> Result<VMData<'lib>, RuntimeError> {
    let val = state.stack.pop_with_rc(state.object_map)?.as_i64();
    Ok(VMData::new_i64(val.abs()))
}

pub fn pow<'lib>(state: VMState) -> Result<VMData<'lib>, RuntimeError> {
    let exponent = state.stack.pop_with_rc(state.object_map)?.as_i64();
    let base = state.stack.pop_with_rc(state.object_map)?.as_i64();
    Ok(VMData::new_i64(base.pow(exponent as u32)))
}

pub fn sqrt<'lib>(state: VMState) -> Result<VMData<'lib>, RuntimeError> {
    let val = state.stack.pop_with_rc(state.object_map)?.as_f64();
    Ok(VMData::new_f64(val.sqrt()))
}

pub fn min<'lib>(state: VMState) -> Result<VMData<'lib>, RuntimeError> {
    let v1 = state.stack.pop_with_rc(state.object_map)?.as_i64();
    let v2 = state.stack.pop_with_rc(state.object_map)?.as_i64();
    Ok(VMData::new_i64(std::cmp::min(v1, v2)))
}

pub fn max<'lib>(state: VMState) -> Result<VMData<'lib>, RuntimeError> {
    let v1 = state.stack.pop_with_rc(state.object_map)?.as_i64();
    let v2 = state.stack.pop_with_rc(state.object_map)?.as_i64();
    Ok(VMData::new_i64(std::cmp::max(v1, v2)))
}

pub fn round<'lib>(state: VMState) -> Result<VMData<'lib>, RuntimeError> {
    let val = state.stack.pop_with_rc(state.object_map)?.as_f64();
    Ok(VMData::new_i64(val.round() as i64))
}

pub fn random<'lib>(state: VMState) -> Result<VMData<'lib>, RuntimeError> {
    let range = (state.stack.pop_with_rc(state.object_map)?.as_i64(), state.stack.pop_with_rc(state.object_map)?.as_i64());
    let mut rng = rng();
    let random = rng.random_range(range.1..range.0);
    Ok(VMData::new_i64(random))
}
