// All of this is assuming that Time is of type: Time(sec: i64, nsec: i64)
// Time will have a tag of 256 as it will be the first type defined by the compiler (0-255 are reserved for the compiler)

use crate::{
    errors::RuntimeError,
    memory::{object_map::Object, vm_data::VMData},
    runtime::vm_state::VMState,
    CallBack,
};

use crate::memory::object_map::Structure;
use time::{format_description, OffsetDateTime};

pub const TIME_ATLAS: &str = include_str!("../../../../libraries/std/time.atlas");

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

    let obj_idx = state.object_map.put(Object::Structure(Structure {
        fields: vec![VMData::new_i64(sec as i64), VMData::new_i64(nsec as i64)],
    }));
    match obj_idx {
        Ok(index) => Ok(VMData::new_object(25, index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

//format_time_iso(time: &Time) -> &string
pub fn format_time_iso(state: VMState) -> Result<VMData, RuntimeError> {
    let time_ptr = state.stack.pop()?.as_object();
    let time_obj = state.object_map.get(time_ptr).structure();

    let sec = time_obj.fields[0].as_i64();
    let nsec = time_obj.fields[1].as_i64();

    let time =
        OffsetDateTime::from_unix_timestamp(sec).unwrap() + time::Duration::nanoseconds(nsec);

    let fmt =
        format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second].[frac][offset]")
            .unwrap();
    let formatted = time.format(&fmt).unwrap();

    let obj_idx = state.object_map.put(Object::String(formatted));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

//format_time(time: &Time, format: &string) -> &string
pub fn format_time(state: VMState) -> Result<VMData, RuntimeError> {
    let format_ptr = state.stack.pop()?.as_object(); // a string is an object
    let time_ptr = state.stack.pop()?.as_object();

    let fmt_str = state.object_map.get(format_ptr).string();
    let time_obj = state.object_map.get(time_ptr).structure();

    let sec = time_obj.fields[0].as_i64();
    let nsec = time_obj.fields[1].as_i64();

    let time =
        OffsetDateTime::from_unix_timestamp(sec).unwrap() + time::Duration::nanoseconds(nsec);

    let fmt = format_description::parse(fmt_str).unwrap();
    let formatted = time.format(&fmt).unwrap();

    let obj_idx = state.object_map.put(Object::String(formatted));
    match obj_idx {
        Ok(index) => Ok(VMData::new_string(index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}

// elapsed(start: &Time, end: &Time) -> &Time
pub fn elapsed(state: VMState) -> Result<VMData, RuntimeError> {
    let end_ptr = state.stack.pop()?.as_object();
    let start_ptr = state.stack.pop()?.as_object();

    let start_obj = state.object_map.get(start_ptr).structure();
    let end_obj = state.object_map.get(end_ptr).structure();

    let start_sec = start_obj.fields[0].as_i64();
    let start_nsec = start_obj.fields[1].as_i64();

    let end_sec = end_obj.fields[0].as_i64();
    let end_nsec = end_obj.fields[1].as_i64();

    let elapsed_sec = end_sec - start_sec;
    let elapsed_nsec = end_nsec - start_nsec;

    let obj_idx = state.object_map.put(Object::Structure(Structure {
        fields: vec![VMData::new_i64(elapsed_sec), VMData::new_i64(elapsed_nsec)],
    }));

    match obj_idx {
        Ok(index) => Ok(VMData::new_object(25, index)),
        Err(_) => Err(RuntimeError::OutOfMemory),
    }
}
