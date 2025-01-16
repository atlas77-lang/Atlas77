pub mod errors;
pub mod instruction;
pub mod vm_state;

use std::collections::HashMap;

use instruction::{Instruction, Program};

use crate::atlas_memory::{object_map::Memory, stack::Stack, vm_data::VMData};

pub type RuntimeResult = Result<(), String>;

pub struct Atlas77VM<'run> {
    pub program: Program<'run>,
    pub stack: Stack,
    stack_frame: Vec<(usize, usize)>, //previous pc and previous stack top
    pub object_map: Memory,
    pub varmap: Vec<HashMap<String, VMData>>, //need to be changed
    pub pc: usize,
}

impl<'run> Atlas77VM<'run> {
    pub fn new(program: Program<'run>) -> Self {
        Self {
            program,
            stack: Stack::new(),
            stack_frame: Vec::new(),
            object_map: Memory::new(1024),
            varmap: vec![HashMap::new()],
            pc: 0,
        }
    }
    pub fn run(&mut self) -> RuntimeResult {
        let label = self
            .program
            .labels
            .iter()
            .find(|label| label.name == self.program.entry_point);
        if let Some(label) = label {
            self.pc = label.position;
        } else {
            return Err("No entry point found".to_string());
        }
        while self.pc < self.program.len() {
            let instr = self.program[self.pc].clone();
            self.execute_instruction(instr)?;
        }
        Ok(())
    }
    pub fn execute_instruction(&mut self, instr: Instruction<'run>) -> RuntimeResult {
        match instr {
            Instruction::PushInt(i) => {
                let val = VMData::new_i64(i);
                self.stack.push(val).unwrap();
                self.pc += 1;
            }
            Instruction::Lt => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_bool(b.as_i64() < a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::Lte => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_bool(b.as_i64() <= a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::JmpZ { pos } => {
                let cond = self.stack.pop().unwrap();
                if !cond.as_bool() {
                    self.pc += pos + 1;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jmp { pos } => {
                self.pc += pos;
            }
            Instruction::StoreI64 { var_name } => {
                let val = self.stack.pop().unwrap();
                self.varmap.last_mut().unwrap().insert(var_name, val);
                self.pc += 1;
            }
            Instruction::LoadI64 { var_name } => {
                let val = self.varmap.last().unwrap().get(&var_name).unwrap();
                self.stack.push(val.clone()).unwrap();
                self.pc += 1;
            }
            Instruction::MulI64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_i64(b.as_i64() * a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::AddI64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_i64(b.as_i64() + a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::SubI64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_i64(b.as_i64() - a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::CallFunction { name, args } => {
                let label = self
                    .program
                    .labels
                    .iter()
                    .find(|label| label.name == name)
                    .unwrap();
                let (pc, sp) = (self.pc, self.stack.top - args as usize);
                self.stack_frame.push((pc, sp));
                self.varmap.push(HashMap::new());
                self.pc = label.position;
            }
            Instruction::Return => {
                let (pc, sp) = self.stack_frame.pop().unwrap_or_else(|| {
                    eprintln!(
                        "No stack frame to return to {:?} @ {}",
                        self.stack.pop(),
                        self.pc
                    );
                    std::process::exit(1);
                });
                self.varmap.pop();
                self.pc = pc + 1;
                let ret = self.stack.pop().unwrap();
                self.stack.truncate(sp);
                self.stack.push(ret).unwrap();
            }
            _ => unimplemented!("{:?}", instr),
        }
        Ok(())
    }
}
