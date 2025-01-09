use crate::{
    atlas_memory::{object_map::Object, vm_data::VMData},
    atlas_runtime::{errors::RuntimeError, vm_state::VMState},
};

// map/for_each/filter/reduce/find won't be coming in this version of the sdlib as it requires fn pointers and closures

// len(list: &List[T]) -> int
pub fn len(state: VMState) -> Result<VMData, RuntimeError> {
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get(list_ptr).list();
    Ok(VMData::new_i64(list.len() as i64))
}

// get(list: &List[T], index: int) -> T
pub fn get(state: VMState) -> Result<VMData, RuntimeError> {
    let index = state.stack.pop()?.as_i64();
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get(list_ptr).list();
    Ok(list[index as usize])
}

// set(list: &List[T], index: int, value: T) -> ()
pub fn set(state: VMState) -> Result<VMData, RuntimeError> {
    let value = state.stack.pop()?;
    let index = state.stack.pop()?.as_i64();
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    list[index as usize] = value;
    Ok(VMData::new_unit())
}

// push(list: &List[T], value: T) -> ()
pub fn push(state: VMState) -> Result<VMData, RuntimeError> {
    let value = state.stack.pop()?;
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    list.push(value);
    Ok(VMData::new_unit())
}

// pop(list: &List[T]) -> T
pub fn pop(state: VMState) -> Result<VMData, RuntimeError> {
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    Ok(list.pop().unwrap())
}

// remove(list: &List[T], index: int) -> T
pub fn remove(state: VMState) -> Result<VMData, RuntimeError> {
    let index = state.stack.pop()?.as_i64();
    let list_ptr = state.stack.pop()?.as_object();
    let list = state.object_map.get_mut(list_ptr).list_mut();
    Ok(list.remove(index as usize))
}

// slice(list: &List[T], start: int, end: int) -> &List[T]
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
