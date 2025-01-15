use crate::{
    atlas_memory::{object_map::Object, vm_data::VMData},
    atlas_vm::{errors::RuntimeError, vm_state::VMState},
};

// List[string] will have a tag of 257 (0-255 are reserved for the compiler)

// read_dir(path: &string) -> &List[string]
pub fn read_dir(state: VMState) -> Result<VMData, RuntimeError> {
    let path_ptr = state.stack.pop()?.as_object();
    let path = state.object_map.get(path_ptr).string();

    let entries = std::fs::read_dir(path).unwrap();
    let mut list = Vec::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        let obj_idx = state.object_map.put(Object::String(path_str.to_string()));
        match obj_idx {
            Ok(index) => list.push(VMData::new_string(index)),
            Err(_) => return Err(RuntimeError::OutOfMemory),
        }
    }

    let list_idx = state.object_map.put(Object::List(list));
    match list_idx {
        Ok(index) => Ok(VMData::new_list(257, index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

// read_file(path: &string) -> &string
pub fn read_file(state: VMState) -> Result<VMData, RuntimeError> {
    let path_ptr = state.stack.pop()?.as_object();
    let path = state.object_map.get(path_ptr).string();

    let content = std::fs::read_to_string(path).unwrap();
    let obj_idx = state.object_map.put(Object::String(content));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

// write_file(path: &string, content: &string) -> ()
pub fn write_file(state: VMState) -> Result<VMData, RuntimeError> {
    let content_ptr = state.stack.pop()?.as_object();
    let path_ptr = state.stack.pop()?.as_object();

    let path = state.object_map.get(path_ptr).string();
    let content = state.object_map.get(content_ptr).string();

    std::fs::write(path, content).unwrap();
    Ok(VMData::new_unit())
}

// file_exists(path: &string) -> bool
pub fn file_exists(state: VMState) -> Result<VMData, RuntimeError> {
    let path_ptr = state.stack.pop()?.as_object();
    let path = state.object_map.get(path_ptr).string();

    let exists = std::path::Path::new(&path).exists();
    Ok(VMData::new_bool(exists))
}

// remove_file(path: &string) -> ()
pub fn remove_file(state: VMState) -> Result<VMData, RuntimeError> {
    let path_ptr = state.stack.pop()?.as_object();
    let path = state.object_map.get(path_ptr).string();

    std::fs::remove_file(path).unwrap();
    Ok(VMData::new_unit())
}
