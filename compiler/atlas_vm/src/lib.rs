pub mod errors;
mod libraries;
pub mod memory;
pub mod runtime;

use std::collections::HashMap;

use errors::RuntimeError;
use runtime::instruction::{Instruction, Program};

use crate::{
    libraries::{
        fs::FILE_FUNCTIONS, io::IO_FUNCTIONS, list::LIST_FUNCTIONS, math::MATH_FUNCTIONS,
        string::STRING_FUNCTIONS, time::TIME_FUNCTIONS,
    },
    memory::{object_map::Memory, stack::Stack, vm_data::VMData},
};

pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type CallBack = fn(runtime::vm_state::VMState) -> RuntimeResult<VMData>;

pub struct Atlas77VM<'run> {
    pub program: Program,
    pub stack: Stack,
    stack_frame: Vec<(usize, usize)>, //previous pc and previous stack top
    pub(crate) _object_map: Memory,
    pub varmap: Vec<HashMap<String, VMData>>, //need to be changed
    pub extern_fn: HashMap<&'run str, CallBack>,
    pub pc: usize,
}

impl Atlas77VM<'_> {
    pub fn new(program: Program) -> Self {
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
                    _ => panic!("Unknown standard libraries"),
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

        self.stack.extends(
            &self
                .program
                .global
                .function_pool
                .iter()
                .map(|t| VMData::new_fn_ptr(*t))
                .collect::<Vec<_>>(),
        )?;
        println!("fn_ptr in the stack: {:?} & all fn_name: {:?}",
                 self.stack.iter().map(|x| x.as_fn_ptr()).collect::<Vec<_>>(),
                 self.program.labels.iter().map(|x| x.name.clone()).collect::<Vec<_>>()
        );
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
                self.stack.push(val)?;
                self.pc += 1;
            }
            Instruction::PushFloat(f) => {
                let val = VMData::new_f64(f);
                self.stack.push(val)?;
                self.pc += 1;
            }
            Instruction::PushUnsignedInt(u) => {
                let val = VMData::new_u64(u);
                self.stack.push(val)?;
                self.pc += 1;
            }
            Instruction::Lt => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_bool(b.as_i64() < a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::Lte => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_bool(b.as_i64() <= a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::Gt => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_bool(b.as_i64() > a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::Gte => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_bool(b.as_i64() >= a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::Eq => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_bool(b.as_i64() == a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::Neq => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_bool(b.as_i64() != a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::JmpZ { pos } => {
                let cond = self.stack.pop()?;
                if !cond.as_bool() {
                    self.pc += (pos + 1) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jmp { pos } => {
                self.pc = (self.pc as isize + pos) as usize;
            }
            Instruction::StoreInteger { var_name } => {
                let val = self.stack.pop()?;
                self.varmap.last_mut().unwrap().insert(var_name, val);
                self.pc += 1;
            }
            Instruction::StoreFloat { var_name } => {
                let val = self.stack.pop()?;
                self.varmap.last_mut().unwrap().insert(var_name, val);
                self.pc += 1;
            }
            Instruction::StoreUnsignedInteger { var_name } => {
                let val = self.stack.pop()?;
                self.varmap.last_mut().unwrap().insert(var_name, val);
                self.pc += 1;
            }
            Instruction::LoadInteger { var_name } => {
                let val = self.varmap.last().unwrap().get(&var_name).unwrap();
                self.stack.push(*val)?;
                self.pc += 1;
            }
            Instruction::LoadFloat { var_name } => {
                let val = self.varmap.last().unwrap().get(&var_name).unwrap();
                self.stack.push(*val)?;
                self.pc += 1;
            }
            Instruction::LoadUnsignedInteger { var_name } => {
                let val = self.varmap.last().unwrap().get(&var_name).unwrap();
                self.stack.push(*val)?;
                self.pc += 1;
            }
            Instruction::IMul => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_i64(b.as_i64() * a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::FMul => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_f64(b.as_f64() * a.as_f64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::UIMul => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_u64(b.as_u64() * a.as_u64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::IDiv => {
                let a = self.stack.pop()?;
                if a == VMData::new_i64(0) {
                    return Err(RuntimeError::DivisionByZero);
                }
                let b = self.stack.pop()?;
                let res = VMData::new_i64(b.as_i64() / a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::FDiv => {
                let a = self.stack.pop()?;
                if a == VMData::new_f64(0.0) {
                    return Err(RuntimeError::DivisionByZero);
                }
                let b = self.stack.pop()?;
                let res = VMData::new_f64(b.as_f64() / a.as_f64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::UIDiv => {
                let a = self.stack.pop()?;
                if a == VMData::new_u64(0) {
                    return Err(RuntimeError::DivisionByZero);
                }
                let b = self.stack.pop()?;
                let res = VMData::new_u64(b.as_u64() / a.as_u64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::IAdd => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_i64(b.as_i64() + a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::FAdd => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_f64(b.as_f64() + a.as_f64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::UIAdd => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_u64(b.as_u64() + a.as_u64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::ISub => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_i64(b.as_i64() - a.as_i64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::FSub => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_f64(b.as_f64() - a.as_f64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::UISub => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_u64(b.as_u64() - a.as_u64());
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::IMod => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let res = VMData::new_i64(b.as_i64() % a.as_i64());
                self.stack.push(res)?;
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
                self.stack.push(res)?;
                self.pc += 1;
            }
            Instruction::DirectCall { pos, args } => {
                let fn_ptr = self.stack[pos];
                let (pc, sp) = (self.pc, self.stack.top - args as usize);
                self.stack_frame.push((pc, sp));
                self.varmap.push(HashMap::new());
                self.pc = fn_ptr.as_fn_ptr();
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
                let ret = self.stack.pop()?;
                self.stack.truncate(sp);
                self.stack.push(ret)?;
            }
            Instruction::Halt => {
                self.pc = self.program.len();
            }
            _ => unimplemented!("{:?}", instr),
        }
        Ok(())
    }
}
