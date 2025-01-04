use atlas_memory::vm_data::VMData;
use rand::{thread_rng, Rng};

use crate::vm_state::VMState;

//abs(x: int) -> int
pub fn abs(state: VMState) -> VMData {
    let val = state.stack.pop().unwrap().as_i64();
    VMData::new_i64(val.abs())
}
//pow(base: int, exponent: int) -> int
pub fn pow(state: VMState) -> VMData {
    let exponent = state.stack.pop().unwrap().as_i64();
    let base = state.stack.pop().unwrap().as_i64();
    VMData::new_i64(base.pow(exponent as u32))
}
//sqrt(x: float) -> float
pub fn sqrt(state: VMState) -> VMData {
    let val = state.stack.pop().unwrap().as_f64();
    VMData::new_f64(val.sqrt())
}
//min(a: int, b: int) -> int
pub fn min(state: VMState) -> VMData {
    let v1 = state.stack.pop().unwrap().as_i64();
    let v2 = state.stack.pop().unwrap().as_i64();
    VMData::new_i64(std::cmp::min(v1, v2))
}
//max(a: int, b: int) -> int
pub fn max(state: VMState) -> VMData {
    let v1 = state.stack.pop().unwrap().as_i64();
    let v2 = state.stack.pop().unwrap().as_i64();
    VMData::new_i64(std::cmp::max(v1, v2)) 
}
//round(x: float) -> int
pub fn round(state: VMState) -> VMData {
    let val = state.stack.pop().unwrap().as_f64();
    VMData::new_i64(val.round() as i64)
}
//random(min: int, max: int) -> int
pub fn random(state: VMState) -> Result<VMData, ()> {
    let range = (
        state.stack.pop().unwrap().as_i64(),
        state.stack.pop().unwrap().as_i64(),
    );
    let mut rng = thread_rng();
    let random = rng.gen_range(range.1..range.0);
    Ok(VMData::new_i64(random))
}