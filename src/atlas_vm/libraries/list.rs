use crate::atlas_vm::errors::RuntimeError;
use crate::atlas_vm::memory::object_map::ObjectKind;
use crate::atlas_vm::memory::vm_data::VMData;
use crate::atlas_vm::runtime::vm_state::VMState;
use crate::atlas_vm::CallBack;

pub const LIST_FUNCTIONS: [(&str, CallBack); 5] = [
    ("len", len),
    ("push", push),
    ("pop", pop),
    ("remove", remove),
    ("slice", slice),
];

pub fn len(state: VMState) -> Result<VMData, RuntimeError> {
    let list_ptr = state.stack.pop()?.as_object();
    let raw_list = state.object_map.get(list_ptr)?;
    let list = raw_list.list();
    Ok(VMData::new_i64(list.len() as i64))
}


pub fn push(state: VMState) -> Result<VMData, RuntimeError> {
    let value = state.stack.pop_with_rc(state.object_map)?;
    let list_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let list = state.object_map.get_mut(list_ptr)?.list_mut();
    list.push(value);
    Ok(VMData::new_unit())
}

pub fn pop(state: VMState) -> Result<VMData, RuntimeError> {
    let list_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let list = state.object_map.get_mut(list_ptr)?.list_mut();
    Ok(list.pop().unwrap())
}

pub fn remove(state: VMState) -> Result<VMData, RuntimeError> {
    let index = state.stack.pop_with_rc(state.object_map)?.as_i64();
    let list_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let list = state.object_map.get_mut(list_ptr)?.list_mut();
    Ok(list.remove(index as usize))
}

pub fn slice(state: VMState) -> Result<VMData, RuntimeError> {
    let end = state.stack.pop_with_rc(state.object_map)?.as_i64();
    let start = state.stack.pop_with_rc(state.object_map)?.as_i64();
    let list_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let raw_list = state.object_map.get(list_ptr)?;
    let list = raw_list.list();
    let sliced = list[start as usize..end as usize].to_vec();
    let obj_idx = state.object_map.put(ObjectKind::List(sliced));
    match obj_idx {
        Ok(index) => Ok(VMData::new_list(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
