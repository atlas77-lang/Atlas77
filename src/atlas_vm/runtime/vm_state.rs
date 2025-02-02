use std::collections::HashMap;

use crate::atlas_vm::memory::varmap::VarMap;
use crate::atlas_vm::memory::{object_map::Memory, stack::Stack, vm_data::VMData};
use crate::atlas_vm::runtime::arena::RuntimeArena;

pub struct VMState<'state, 'run> {
    pub stack: &'state mut Stack,
    pub object_map: &'state mut Memory<'run>,
    pub consts: &'state HashMap<&'run str, VMData>,
    pub var_map: &'state mut VarMap<'run>,
    pub runtime_arena: &'state RuntimeArena<'run>
}

impl<'state, 'run> VMState<'state, 'run> {
    pub fn new(
        stack: &'state mut Stack,
        object_map: &'state mut Memory,
        consts: &'state HashMap<&'run str, VMData>,
        var_map: &'state mut VarMap<'run>,
        runtime_arena: &'state RuntimeArena<'run>
    ) -> Self {
        Self {
            stack,
            object_map,
            consts,
            var_map,
            runtime_arena,
        }
    }
}
