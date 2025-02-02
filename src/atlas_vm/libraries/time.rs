// All of this is assuming that Time is of type: Time(sec: i64, nsec: i64)
// Time will have a tag of 256 as it will be the first type defined by the compiler (0-255 are reserved for the compiler)

use std::collections::HashMap;
use crate::atlas_vm::errors::RuntimeError;
use crate::atlas_vm::memory::object_map::{ObjectKind, Class};
use crate::atlas_vm::memory::vm_data::VMData;
use crate::atlas_vm::runtime::vm_state::VMState;
use crate::atlas_vm::CallBack;
use time::{format_description, OffsetDateTime};

pub const TIME_FUNCTIONS: [(&str, CallBack); 4] = [
    ("now", now),
    ("format_time_iso", format_time_iso),
    ("format_time", format_time),
    ("elapsed", elapsed),
];

//now() -> &Time
pub fn now(state: VMState) -> Result<VMData, RuntimeError> {
    let time = std::time::SystemTime::now();
    let duration = time.duration_since(std::time::UNIX_EPOCH).unwrap();

    let sec = duration.as_secs();
    let nsec = duration.subsec_nanos();

    let mut fields = HashMap::new();
    fields.insert("sec", VMData::new_i64(sec as i64));
    fields.insert("nsec", VMData::new_i64(nsec as i64));

    let obj_idx = state.object_map.put(ObjectKind::Class(Class {
        fields,
    }));
    match obj_idx {
        Ok(index) => Ok(VMData::new_object(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

//format_time_iso(time: &Time) -> &string
pub fn format_time_iso(state: VMState) -> Result<VMData, RuntimeError> {
    let time_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let raw_time_obj = state.object_map.get(time_ptr)?;
    let time_obj = raw_time_obj.structure();

    let sec = time_obj.fields.get("sec").unwrap().as_i64();
    let nsec = time_obj.fields.get("nsec").unwrap().as_i64();

    let time =
        OffsetDateTime::from_unix_timestamp(sec).unwrap() + time::Duration::nanoseconds(nsec);

    let fmt =
        format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second].[frac][offset]")
            .unwrap();
    let formatted = time.format(&fmt).unwrap();

    let obj_idx = state.object_map.put(ObjectKind::String(state.runtime_arena.alloc(formatted)));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

//format_time(time: &Time, format: &string) -> &string
pub fn format_time(state: VMState) -> Result<VMData, RuntimeError> {
    let format_ptr = state.stack.pop_with_rc(state.object_map)?.as_object(); // a string is an object
    let time_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();

    let fmt_str = &state.object_map.get(format_ptr)?.string().clone();
    let raw_time_obj = state.object_map.get(time_ptr)?;
    let time_obj = raw_time_obj.structure();

    let sec = time_obj.fields.get("sec").unwrap().as_i64();
    let nsec = time_obj.fields.get("nsec").unwrap().as_i64();

    let time =
        OffsetDateTime::from_unix_timestamp(sec).unwrap() + time::Duration::nanoseconds(nsec);

    let fmt = format_description::parse(fmt_str).unwrap();
    let formatted = time.format(&fmt).unwrap();

    let obj_idx = state.object_map.put(ObjectKind::String(state.runtime_arena.alloc(formatted)));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

// elapsed(start: &Time, end: &Time) -> &Time
pub fn elapsed(state: VMState) -> Result<VMData, RuntimeError> {
    let end_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();
    let start_ptr = state.stack.pop_with_rc(state.object_map)?.as_object();

    let start_obj = &state.object_map.get(start_ptr)?.structure().clone();
    let raw_time_obj = state.object_map.get(end_ptr)?;
    let end_obj = raw_time_obj.structure();

    let start_sec = start_obj.fields.get("sec").unwrap().as_i64();
    let start_nsec = start_obj.fields.get("nsec").unwrap().as_i64();

    let end_sec = end_obj.fields.get("sec").unwrap().as_i64();
    let end_nsec = end_obj.fields.get("nsec").unwrap().as_i64();

    let elapsed_sec = end_sec - start_sec;
    let elapsed_nsec = end_nsec - start_nsec;

    let mut fields = HashMap::new();
    fields.insert("sec", VMData::new_i64(elapsed_sec as i64));
    fields.insert("nsec", VMData::new_i64(elapsed_nsec as i64));

    let obj_idx = state.object_map.put(ObjectKind::Class(Class {
        fields,
    }));

    match obj_idx {
        Ok(index) => Ok(VMData::new_object(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
