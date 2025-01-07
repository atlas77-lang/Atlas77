use std::collections::HashMap;

use crate::atlas_memory::vm_data::VMData;

pub struct VarMap {
    pub map: HashMap<String, VMData>,
}
