use crate::errors::RuntimeError;
use crate::memory::object_map::Object;
use crate::memory::vm_data::VMData;
use crate::runtime::vm_state::VMState;
use crate::CallBack;

pub const FILE_FUNCTIONS: [(&str, CallBack); 5] = [
    ("read_dir", read_dir),
    ("read_file", read_file),
    ("write_file", write_file),
    ("file_exists", file_exists),
    ("remove_file", remove_file),
];

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

pub fn write_file(state: VMState) -> Result<VMData, RuntimeError> {
    let content_ptr = state.stack.pop()?.as_object();
    let path_ptr = state.stack.pop()?.as_object();

    let path = state.object_map.get(path_ptr).string();
    let content = state.object_map.get(content_ptr).string();

    std::fs::write(path, content).unwrap();
    Ok(VMData::new_unit())
}

pub fn file_exists(state: VMState) -> Result<VMData, RuntimeError> {
    let path_ptr = state.stack.pop()?.as_object();
    let path = state.object_map.get(path_ptr).string();

    let exists = std::path::Path::new(&path).exists();
    Ok(VMData::new_bool(exists))
}

pub fn remove_file(state: VMState) -> Result<VMData, RuntimeError> {
    let path_ptr = state.stack.pop()?.as_object();
    let path = state.object_map.get(path_ptr).string();

    std::fs::remove_file(path).unwrap();
    Ok(VMData::new_unit())
}
