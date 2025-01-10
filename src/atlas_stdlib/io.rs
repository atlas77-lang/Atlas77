use crate::{
    atlas_memory::{object_map::Object, vm_data::VMData},
    atlas_runtime::{errors::RuntimeError, vm_state::VMState},
};

// println<T>(value: T) -> ()
pub fn println(state: VMState) -> Result<VMData, RuntimeError> {
    let val = state.stack.pop()?;
    match val.tag {
        VMData::TAG_BOOL | VMData::TAG_U64 | VMData::TAG_I64 | VMData::TAG_FLOAT => {
            println!("{}", val)
        }
        VMData::TAG_STR => {
            println!("{}", state.object_map.get(val.as_object()).string())
        }
        _ => {
            println!("{}", state.object_map.get(val.as_object()))
        }
    }
    Ok(VMData::new_unit())
}

// print<T>(value: T) -> ()
pub fn print(state: VMState) -> Result<VMData, RuntimeError> {
    let val = state.stack.pop()?;
    match val.tag {
        VMData::TAG_BOOL | VMData::TAG_U64 | VMData::TAG_I64 | VMData::TAG_FLOAT => {
            print!("{}", val)
        }
        VMData::TAG_STR => {
            print!("{}", state.object_map.get(val.as_object()).string())
        }
        _ => {
            print!("{}", state.object_map.get(val.as_object()))
        }
    }
    Ok(VMData::new_unit())
}

// input() -> &string
pub fn input(state: VMState) -> Result<VMData, RuntimeError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let obj_index = state
        .object_map
        .put(Object::String(input.trim().to_string()));
    match obj_index {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
