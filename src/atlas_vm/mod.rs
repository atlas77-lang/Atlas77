pub mod instruction;
pub mod vm_state;

use std::collections::HashMap;

use instruction::Program;

use crate::atlas_memory::{object_map::Memory, stack::Stack};

pub struct Atlas77VM<'run> {
    pub program: Program<'run>,
    pub stack: Stack,
    pub object_map: Memory,
    pub varmap: HashMap<String, usize>, //need to be changed
    pub pc: usize,
}

impl<'run> Atlas77VM<'run> {
    pub fn new(program: Program<'run>) -> Self {
        Self {
            program,
            stack: Stack::new(),
            object_map: Memory::new(1024),
            varmap: HashMap::new(),
            pc: 0,
        }
    }
    pub fn run(&mut self) {
        while self.pc < self.program.len() {}
    }
}
