pub mod instruction;
pub mod vm_state;
pub mod errors;

use std::collections::HashMap;

use instruction::{Instruction, Program};

use crate::atlas_memory::{object_map::Memory, stack::Stack, varmap::Varmap, vm_data::VMData};

pub type RuntimeResult = Result<(), String>;

pub struct Atlas77VM<'run> {
    pub program: Program<'run>,
    pub stack: Stack,
    pub object_map: Memory,
    pub varmap: Varmap<'run, String, VMData>, //need to be changed
    pub pc: usize,
}

impl<'run> Atlas77VM<'run> {
    pub fn new(program: Program<'run>) -> Self {
        Self {
            program,
            stack: Stack::new(),
            object_map: Memory::new(1024),
            varmap: <Varmap<String, VMData>>::default(),
            pc: 0,
        }
    }
    pub fn run(&mut self) -> RuntimeResult {
        //For test purposes
        let _ = self.stack.push(VMData::new_i64(54));
        while self.pc < self.program.len() {
            let instr = self.program[self.pc].clone();
            self.execute_instruction(instr)?;
            self.pc += 1;
        }
        Ok(())
    }
    pub fn execute_instruction(&mut self, instr: Instruction<'run>) -> RuntimeResult {
        match instr {
            Instruction::StoreI64 { var_name } => {
                let val = self.stack.pop().unwrap();
                self.varmap.insert(var_name, val);
            }
            Instruction::LoadI64 { var_name } => {
                let val = self.varmap.get(var_name).unwrap();
                self.stack.push(val.clone()).unwrap();
            }
            Instruction::MulI64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_i64(a.as_i64() * b.as_i64());
                self.stack.push(res).unwrap();
            }
            Instruction::Return => {
                let val = self.stack.pop().unwrap();
                println!("{}", val);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }
}
