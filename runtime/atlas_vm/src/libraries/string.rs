use crate::errors::RuntimeError;
use crate::memory::object_map::ObjectKind;
use crate::memory::vm_data::VMData;
use crate::runtime::vm_state::VMState;
use crate::CallBack;

pub const STRING_FUNCTIONS: [(&str, CallBack); 5] = [
    ("str_len", str_len),
    ("trim", trim),
    ("to_upper", to_upper),
    ("to_lower", to_lower),
    ("split", split),
];

pub fn str_len(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let raw_string = state.object_map.get(string_ptr);
    let string = raw_string.string();
    Ok(VMData::new_i64(string.len() as i64))
}

pub fn trim(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let string = state.object_map.get(string_ptr).string().clone();

    let trimmed = string.trim();

    let obj_idx = state.object_map.put(ObjectKind::String(trimmed.to_string()));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

pub fn to_upper(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let raw_string = state.object_map.get(string_ptr);
    let string = raw_string.string();

    let upper = string.to_uppercase();

    let obj_idx = state.object_map.put(ObjectKind::String(upper));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

pub fn to_lower(state: VMState) -> Result<VMData, RuntimeError> {
    let string_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let raw_string = state.object_map.get(string_ptr);
    let string = raw_string.string();

    let lower = string.to_lowercase();

    let obj_idx = state.object_map.put(ObjectKind::String(lower));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

pub fn split(state: VMState) -> Result<VMData, RuntimeError> {
    let delimiter_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let string_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();

    let delimiter = &state.object_map.get(delimiter_ptr).string().clone();
    let raw_string = state.object_map.get(string_ptr);
    let string = raw_string.string();

    let split_strings: Vec<String> = string.split(delimiter).map(|s| s.to_string()).collect();
    let list: Vec<VMData> = split_strings
        .into_iter()
        .map(|s| {
            let obj_idx = match state.object_map.put(ObjectKind::String(s)) {
                Ok(index) => index,
                Err(_) => panic!("Out of memory"),
            };
            VMData::new_string(obj_idx)
        })
        .collect();

    let list_idx = state.object_map.put(ObjectKind::List(list));
    match list_idx {
        Ok(index) => Ok(VMData::new_list(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
