use std::collections::HashMap;

use crate::atlas_vm::memory::{object_map::Memory, stack::Stack, vm_data::VMData};
use crate::atlas_vm::runtime::arena::RuntimeArena;

pub struct VMState<'state, 'run> {
    pub stack: &'state mut Stack<'run>,
    pub object_map: &'state mut Memory<'run>,
    pub consts: &'state HashMap<&'run str, VMData<'run>>,
    pub runtime_arena: &'state RuntimeArena<'run>,
}

impl<'state, 'run> VMState<'state, 'run> {
    pub fn new(
        stack: &'state mut Stack<'run>,
        object_map: &'state mut Memory<'run>,
        consts: &'state HashMap<&'run str, VMData<'run>>,
        runtime_arena: &'state RuntimeArena<'run>,
    ) -> Self {
        Self {
            stack,
            object_map,
            consts,
            runtime_arena,
        }
    }
}
