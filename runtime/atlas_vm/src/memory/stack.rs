use crate::memory::object_map::Memory;
use crate::{errors::RuntimeError, memory::vm_data::VMData, RuntimeResult};
use std::fmt::Display;
use std::ops::Index;

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
            top: 0,
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
    pub fn push_with_rc(&mut self, val: VMData, mem: &mut Memory) -> Result<(), RuntimeError> {
        if self.top < STACK_SIZE {
            self.values[self.top] = val;
            match val.tag {
                VMData::TAG_OBJECT | VMData::TAG_LIST | VMData::TAG_STR => {
                    mem.rc_inc(val.as_object());
                }
                _ => {}
            }
            self.top += 1;
            Ok(())
        } else {
            Err(RuntimeError::StackOverflow)
        }
    }

    #[inline(always)]
    pub fn truncate(&mut self, new_top: usize, mem: &mut Memory) -> RuntimeResult<()> {
        for i in new_top..=self.top {
            match self.values[i].tag {
                VMData::TAG_OBJECT | VMData::TAG_LIST | VMData::TAG_STR => {
                    mem.rc_dec(self.values[i].as_object())?;
                }
                _ => {}
            }
        }
        self.top = new_top;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<VMData, RuntimeError> {
        if self.top != 0 {
            self.top -= 1;
            Ok(self.values[self.top])
        } else {
            Err(RuntimeError::StackUnderflow)
        }
    }
    pub fn pop_with_rc(&mut self, mem: &mut Memory) -> Result<VMData, RuntimeError> {
        if self.top != 0 {
            self.top -= 1;
            let r = self.values[self.top];
            match r.tag {
                VMData::TAG_OBJECT | VMData::TAG_LIST | VMData::TAG_STR => {
                    mem.rc_dec(r.as_object())?;
                }
                _ => {}
            }
            Ok(r)
        } else {
            Err(RuntimeError::StackUnderflow)
        }
    }

    #[inline(always)]
    pub fn last(&self) -> Result<&VMData, RuntimeError> {
        if self.top != 0 {
            Ok(&self.values[self.top - 1])
        } else {
            Err(RuntimeError::StackUnderflow)
        }
    }

    pub fn extends(&mut self, values: &[VMData]) -> Result<(), RuntimeError> {
        if self.top + values.len() < STACK_SIZE {
            for val in values {
                self.values[self.top] = *val;
                self.top += 1;
            }
            Ok(())
        } else {
            Err(RuntimeError::StackOverflow)
        }
    }

    pub fn push_object(&mut self, _obj: &[VMData]) -> Result<(), RuntimeError> {
        unimplemented!("push_object(&mut self, obj: &[VMData])")
    }

    pub fn new_stack_frame(&mut self) {}

    pub fn set(&mut self, _offset: usize) {}

    pub fn iter(&self) -> std::slice::Iter<VMData> {
        self.values[..self.top].iter()
    }
}

impl IntoIterator for Stack {
    type Item = VMData;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values[..self.top].to_vec().into_iter()
    }
}

impl Index<usize> for Stack {
    type Output = VMData;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stack: {{ values: {}, top: {}}}",
            {
                let mut s = "[".to_string();
                for i in 0..=self.top {
                    s.push_str(&format!("{}, ", self.values[i]))
                }
                s.push(']');
                s
            },
            self.top
        )
    }
}
