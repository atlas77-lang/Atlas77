use std::fmt::Display;

use crate::{errors::RuntimeError, memory::vm_data::VMData};
/// The size of the stack in bytes, 16384 is the maximum before it overflows "thread main"
///
/// I'll try allocating the stack into the heap later on so
const STACK_SIZE: usize = 16 * 16384 / size_of::<VMData>();
/// The stack should be more used overall.
///
/// And allow features such as holding objects themselves e.g.
/// ```rs
/// fn push_object(&mut self, obj: &[VMData]) {}
/// fn access(&mut self, offset: usize) -> VMData {}
/// ```
///
/// The stack should also be able to create new stack frames with special rules
/// to access data in other stack frames
#[derive(Debug)]
pub struct Stack {
    values: [VMData; STACK_SIZE],
    pub top: usize,
}
impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

/// TODO: this implementation should be overhauled a bit cuz it's kinda clunky
impl Stack {
    pub fn new() -> Self {
        Self {
            values: [VMData::new_unit(); STACK_SIZE],
            top: 1,
        }
    }

    pub fn push(&mut self, val: VMData) -> Result<(), RuntimeError> {
        if self.top < STACK_SIZE {
            self.values[self.top] = val;
            self.top += 1;
            Ok(())
        } else {
            Err(RuntimeError::StackOverflow)
        }
    }

    #[inline]
    pub fn truncate(&mut self, new_top: usize) {
        self.top = new_top;
    }

    pub fn pop(&mut self) -> Result<VMData, RuntimeError> {
        if self.top != 0 {
            self.top -= 1;
            let r = self.values[self.top];
            Ok(r)
        } else {
            Err(RuntimeError::StackUndeflow)
        }
    }

    #[inline(always)]
    pub fn last(&self) -> Result<&VMData, RuntimeError> {
        if self.top != 0 {
            Ok(&self.values[self.top - 1])
        } else {
            Err(RuntimeError::StackUndeflow)
        }
    }

    pub fn push_object(&mut self, _obj: &[VMData]) -> Result<(), RuntimeError> {
        unimplemented!("push_object(&mut self, obj: &[VMData])")
    }

    pub fn new_stack_frame(&mut self) {}

    pub fn get(&mut self, _offset: usize) -> Result<VMData, RuntimeError> {
        unimplemented!("get(&mut self, offset: usize)")
    }

    pub fn set(&mut self, _offset: usize) {}
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stack: {{ values: {}, top: {}}}",
            {
                let mut s = "[".to_string();
                for i in 0..self.top - 1 {
                    s.push_str(&format!("{}, ", self.values[i]))
                }
                s.push(']');
                s
            },
            self.top
        )
    }
}
