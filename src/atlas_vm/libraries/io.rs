use crate::atlas_vm::errors::RuntimeError;
use crate::atlas_vm::memory::object_map::ObjectKind;
use crate::atlas_vm::memory::vm_data::VMData;
use crate::atlas_vm::runtime::vm_state::VMState;
use crate::atlas_vm::CallBack;

pub const IO_FUNCTIONS: [(&str, CallBack); 3] = [
    ("println", println),
    ("print", print),
    ("input", input),
];
pub fn println(state: VMState) -> Result<VMData, RuntimeError> {
    let val = state.stack.pop()?;
    match val.tag {
        VMData::TAG_BOOL
        | VMData::TAG_U64
        | VMData::TAG_I64
        | VMData::TAG_FLOAT
        | VMData::TAG_CHAR => {
            println!("{}", val)
        }
        VMData::TAG_STR => {
            println!("{}", state.object_map.get(val.as_object())?.string())
        }
        _ => {
            println!("{}", state.object_map.get(val.as_object())?)
        }
    }
    Ok(VMData::new_unit())
}

pub fn print(state: VMState) -> Result<VMData, RuntimeError> {
    let val = state.stack.pop()?;
    match val.tag {
        VMData::TAG_BOOL
        | VMData::TAG_U64
        | VMData::TAG_I64
        | VMData::TAG_FLOAT
        | VMData::TAG_CHAR => {
            print!("{}", val)
        }
        VMData::TAG_STR => {
            print!("{}", state.object_map.get(val.as_object())?.string())
        }
        _ => {
            print!("{}", state.object_map.get(val.as_object())?)
        }
    }
    Ok(VMData::new_unit())
}

pub fn input(state: VMState) -> Result<VMData, RuntimeError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let obj_index = state
        .object_map
        .put(ObjectKind::String(input.trim().to_string()));
    match obj_index {
        Ok(index) => {
            println!("Input: {}", input.trim());
            Ok(VMData::new_string(index))
        }
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
