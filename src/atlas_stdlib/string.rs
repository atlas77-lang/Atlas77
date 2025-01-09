use crate::{
    atlas_memory::{object_map::Object, vm_data::VMData},
    atlas_runtime::{errors::RuntimeError, vm_state::VMState},
};

// str_len(str: &string)
pub fn str_len(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop()?.as_object();
    let string = state.object_map.get(string_ptr).string();
    Ok(VMData::new_i64(string.len() as i64))
}

// trim(str: &string) -> &string
pub fn trim(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop()?.as_object();
    let string = state.object_map.get(string_ptr).string();

    let trimmed = string.trim();

    let obj_idx = state.object_map.put(Object::String(trimmed.to_string()));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

// to_upper(str: &string) -> &string
pub fn to_upper(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop()?.as_object();
    let string = state.object_map.get(string_ptr).string();

    let upper = string.to_uppercase();

    let obj_idx = state.object_map.put(Object::String(upper));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

// to_lower(str: &string) -> &string
pub fn to_lower(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop()?.as_object();
    let string = state.object_map.get(string_ptr).string();

    let lower = string.to_lowercase();

    let obj_idx = state.object_map.put(Object::String(lower));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

// split(str: &string, delimiter: &string) -> &List[string]
pub fn split(state: VMState) -> Result<VMData, RuntimeError> {
    let delimiter_ptr = state.stack.pop()?.as_object();
    let string_ptr = state.stack.pop()?.as_object();

    let delimiter = state.object_map.get(delimiter_ptr).string();
    let string = state.object_map.get(string_ptr).string();

    let split_strings: Vec<String> = string.split(delimiter).map(|s| s.to_string()).collect();
    let list: Vec<VMData> = split_strings
        .into_iter()
        .map(|s| {
            let obj_idx = match state.object_map.put(Object::String(s)) {
                Ok(index) => index,
                Err(_) => panic!("Out of memory"),
            };
            VMData::new_string(obj_idx)
        })
        .collect();

    let list_idx = state.object_map.put(Object::List(list));
    match list_idx {
        Ok(index) => Ok(VMData::new_list(257, index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
