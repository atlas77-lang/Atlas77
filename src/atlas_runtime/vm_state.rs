use std::collections::HashMap;

use crate::atlas_memory::{object_map::Memory, stack::Stack, vm_data::VMData};

use super::{FuncMap, VarMap};

pub struct VMState<'state, 'run> {
    pub stack: &'state mut Stack,
    pub object_map: &'state mut Memory,
    pub consts: &'state HashMap<&'run str, VMData>,
    pub varmap: &'state VarMap<'run>,
    pub funcmap: &'state FuncMap<'run>,
}

impl<'state, 'run> VMState<'state, 'run> {
    pub fn new(
        stack: &'state mut Stack,
        object_map: &'state mut Memory,
        consts: &'state HashMap<&'run str, VMData>,
        varmap: &'state VarMap<'run>,
        funcmap: &'state FuncMap<'run>,
    ) -> Self {
        Self {
            stack,
            object_map,
            consts,
            varmap,
            funcmap,
        }
    }
}
