use std::collections::HashMap;

use crate::atlas_memory::{object_map::Memory, stack::Stack, vm_data::VMData};

pub struct VMState<'state, 'run> {
    pub stack: &'state mut Stack,
    pub object_map: &'state mut Memory,
    pub consts: &'state HashMap<&'run str, VMData>,
    pub varmap: &'state HashMap<String, VMData>,
}

impl<'state, 'run> VMState<'state, 'run> {
    pub fn new(
        stack: &'state mut Stack,
        object_map: &'state mut Memory,
        consts: &'state HashMap<&'run str, VMData>,
        varmap: &'state HashMap<String, VMData>,
    ) -> Self {
        Self {
            stack,
            object_map,
            consts,
            varmap,
        }
    }
}
