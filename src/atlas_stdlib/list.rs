use crate::{
    atlas_memory::{object_map::Object, vm_data::VMData},
    atlas_vm::{errors::RuntimeError, vm_state::VMState, CallBack},
};

pub const LIST_ATLAS: &str = include_str!("list.atlas");

pub const LIST_FUNCTIONS: [(&str, CallBack); 7] = [
    ("len", len),
    ("get", get),
    ("set", set),
    ("push", push),
    ("pop", pop),
    ("remove", remove),
    ("slice", slice),
];

pub fn len(state: VMState) -> Result<VMData, RuntimeError> {
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get(list_ptr).list();
    Ok(VMData::new_i64(list.len() as i64))
}

pub fn get(state: VMState) -> Result<VMData, RuntimeError> {
    let index = state.stack.pop()?.as_i64();
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get(list_ptr).list();
    Ok(list[index as usize])
}

pub fn set(state: VMState) -> Result<VMData, RuntimeError> {
    let value = state.stack.pop()?;
    let index = state.stack.pop()?.as_i64();
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    list[index as usize] = value;
    Ok(VMData::new_unit())
}

pub fn push(state: VMState) -> Result<VMData, RuntimeError> {
    let value = state.stack.pop()?;
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    list.push(value);
    Ok(VMData::new_unit())
}

pub fn pop(state: VMState) -> Result<VMData, RuntimeError> {
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    Ok(list.pop().unwrap())
}

pub fn remove(state: VMState) -> Result<VMData, RuntimeError> {
    let index = state.stack.pop()?.as_i64();
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    Ok(list.remove(index as usize))
}

pub fn slice(state: VMState) -> Result<VMData, RuntimeError> {
    let end = state.stack.pop()?.as_i64();
    let start = state.stack.pop()?.as_i64();
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get(list_ptr).list();
    let sliced = list[start as usize..end as usize].to_vec();
    let obj_idx = state.object_map.put(Object::List(sliced));
    match obj_idx {
        Ok(index) => Ok(VMData::new_list(257, index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
