use std::collections::HashMap;

use crate::atlas_memory::{object_map::Memory, stack::Stack, vm_data::VMData};
use internment::Intern;

pub struct VMState<'state> {
    pub stack: &'state mut Stack,
    pub object_map: &'state mut Memory,
    pub consts: &'state HashMap<Intern<String>, VMData>,
}

impl<'state> VMState<'state> {
    pub fn new(
        stack: &'state mut Stack,
        object_map: &'state mut Memory,
        consts: &'state HashMap<Intern<String>, VMData>,
    ) -> Self {
        Self {
            stack,
            object_map,
            consts,
        }
    }
}
