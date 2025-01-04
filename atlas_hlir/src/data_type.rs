// This is to keep it in sync with the VMData struct in atlas_memory/src/vm_data.rs
pub trait DataType {
    const UNIT_TYPE: u64 = 0;
    const UINT_TYPE: u64 = 4;
    const INT_TYPE: u64 = 8;
    const FLOAT_TYPE: u64 = 9;
    const BOOL_TYPE: u64 = 10;
    const STR_TYPE: u64 = 11;
    const CHAR_TYPE: u64 = 12;
    const FN_PTR_TYPE: u64 = 13; //FN_PTR might have to be changed because () -> int != (int) -> int for example
}

impl DataType for u64 { }