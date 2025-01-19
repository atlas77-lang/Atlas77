pub mod errors;
pub mod runtime;
pub mod memory;
mod libraries;

use std::collections::HashMap;

use errors::RuntimeError;
use runtime::instruction::{Instruction, Program};

use crate::{
    memory::{object_map::Memory, stack::Stack, vm_data::VMData},
    libraries::{
        file::FILE_FUNCTIONS, io::IO_FUNCTIONS, list::LIST_FUNCTIONS, math::MATH_FUNCTIONS,
        string::STRING_FUNCTIONS, time::TIME_FUNCTIONS,
    },
};

pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type CallBack = fn(runtime::vm_state::VMState) -> RuntimeResult<VMData>;

pub struct Atlas77VM<'run> {
    pub program: Program<'run>,
    pub stack: Stack,
    stack_frame: Vec<(usize, usize)>, //previous pc and previous stack top
    pub(crate) _object_map: Memory,
    pub varmap: Vec<HashMap<String, VMData>>, //need to be changed
    pub extern_fn: HashMap<&'run str, CallBack>,
    pub pc: usize,
}

impl<'run> Atlas77VM<'run> {
    pub fn new(program: Program<'run>) -> Self {
        let mut extern_fn: HashMap<&str, CallBack> = HashMap::new();
        program.libraries.iter().for_each(|lib| {
            if lib.is_std {
                let lib_name = lib.name.split('/').last().unwrap();
                match lib_name {
                    "file" => {
                        FILE_FUNCTIONS.iter().for_each(|(name, func)| {
                            extern_fn.insert(name, *func);
                        });
                    }
                    "io" => {
                        IO_FUNCTIONS.iter().for_each(|(name, func)| {
                            extern_fn.insert(name, *func);
                        });
                    }
                    "list" => {
                        LIST_FUNCTIONS.iter().for_each(|(name, func)| {
                            extern_fn.insert(name, *func);
                        });
                    }
                    "math" => {
                        MATH_FUNCTIONS.iter().for_each(|(name, func)| {
                            extern_fn.insert(name, *func);
                        });
                    }
                    "string" => {
                        STRING_FUNCTIONS.iter().for_each(|(name, func)| {
                            extern_fn.insert(name, *func);
                        });
                    }
                    "time" => {
                        TIME_FUNCTIONS.iter().for_each(|(name, func)| {
                            extern_fn.insert(name, *func);
                        });
                    }
                    _ => panic!("Unknown standard library"),
                }
            }
        });
        Self {
            program,
            stack: Stack::new(),
            stack_frame: Vec::new(),
            _object_map: Memory::new(1024),
            varmap: vec![HashMap::new()],
            extern_fn,
            pc: 0,
        }
    }
    pub fn run(&mut self) -> RuntimeResult<VMData> {
        let label = self
            .program
            .labels
            .iter()
            .find(|label| label.name == self.program.entry_point);
        if let Some(label) = label {
            self.pc = label.position;
        } else {
            return Err(RuntimeError::EntryPointNotFound(
                self.program.entry_point.to_string(),
            ));
        }
        while self.pc < self.program.len() {
            let instr = self.program[self.pc].clone();
            self.execute_instruction(instr)?;
        }
        self.stack.pop()
    }
    /// TODO: Add check for unsigned int
    pub fn execute_instruction(&mut self, instr: Instruction) -> RuntimeResult<()> {
        match instr {
            Instruction::PushInt(i) => {
                let val = VMData::new_i64(i);
                self.stack.push(val).unwrap();
                self.pc += 1;
            }
            Instruction::PushFloat(f) => {
                let val = VMData::new_f64(f);
                self.stack.push(val).unwrap();
                self.pc += 1;
            }
            Instruction::PushUnsignedInt(u) => {
                let val = VMData::new_u64(u);
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
            Instruction::Gt => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_bool(b.as_i64() > a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::Gte => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_bool(b.as_i64() >= a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::Eq => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_bool(b.as_i64() == a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::Neq => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_bool(b.as_i64() != a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::JmpZ { pos } => {
                let cond = self.stack.pop().unwrap();
                if !cond.as_bool() {
                    self.pc += (pos + 1) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jmp { pos } => {
                self.pc = (self.pc as isize + pos) as usize;
            }
            Instruction::StoreI64 { var_name } => {
                let val = self.stack.pop().unwrap();
                self.varmap.last_mut().unwrap().insert(var_name, val);
                self.pc += 1;
            }
            Instruction::StoreF64 { var_name } => {
                let val = self.stack.pop().unwrap();
                self.varmap.last_mut().unwrap().insert(var_name, val);
                self.pc += 1;
            }
            Instruction::StoreU64 { var_name } => {
                let val = self.stack.pop().unwrap();
                self.varmap.last_mut().unwrap().insert(var_name, val);
                self.pc += 1;
            }
            Instruction::LoadI64 { var_name } => {
                let val = self.varmap.last().unwrap().get(&var_name).unwrap();
                self.stack.push(*val).unwrap();
                self.pc += 1;
            }
            Instruction::LoadF64 { var_name } => {
                let val = self.varmap.last().unwrap().get(&var_name).unwrap();
                self.stack.push(*val).unwrap();
                self.pc += 1;
            }
            Instruction::LoadU64 { var_name } => {
                let val = self.varmap.last().unwrap().get(&var_name).unwrap();
                self.stack.push(*val).unwrap();
                self.pc += 1;
            }
            Instruction::MulI64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_i64(b.as_i64() * a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::MulF64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_f64(b.as_f64() * a.as_f64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::MulU64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_u64(b.as_u64() * a.as_u64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::DivI64 => {
                let a = self.stack.pop().unwrap();
                if a == VMData::new_i64(0) {
                    return Err(RuntimeError::DivisionByZero);
                }
                let b = self.stack.pop().unwrap();
                let res = VMData::new_i64(b.as_i64() / a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::DivF64 => {
                let a = self.stack.pop().unwrap();
                if a == VMData::new_f64(0.0) {
                    return Err(RuntimeError::DivisionByZero);
                }
                let b = self.stack.pop().unwrap();
                let res = VMData::new_f64(b.as_f64() / a.as_f64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::DivU64 => {
                let a = self.stack.pop().unwrap();
                if a == VMData::new_u64(0) {
                    return Err(RuntimeError::DivisionByZero);
                }
                let b = self.stack.pop().unwrap();
                let res = VMData::new_u64(b.as_u64() / a.as_u64());
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
            Instruction::AddF64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_f64(b.as_f64() + a.as_f64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::AddU64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_u64(b.as_u64() + a.as_u64());
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
            Instruction::SubF64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_f64(b.as_f64() - a.as_f64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::SubU64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_u64(b.as_u64() - a.as_u64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::ModI64 => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let res = VMData::new_i64(b.as_i64() % a.as_i64());
                self.stack.push(res).unwrap();
                self.pc += 1;
            }
            Instruction::ExternCall { name, .. } => {
                let consts = HashMap::new();
                let vm_state = runtime::vm_state::VMState::new(
                    &mut self.stack,
                    &mut self._object_map,
                    &consts,
                    self.varmap.last().unwrap(),
                );
                let extern_fn: &CallBack = self.extern_fn.get::<&str>(&name.as_str()).unwrap();
                let res = extern_fn(vm_state)?;
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
            Instruction::Halt => {
                self.pc = self.program.len();
            }
            _ => unimplemented!("{:?}", instr),
        }
        Ok(())
    }
}
