use crate::atlas_vm::memory::object_map::Memory;
use crate::atlas_vm::{errors::RuntimeError, memory::vm_data::VMData, RuntimeResult};
use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

/// The size of the stack in bytes
///
/// I'll try allocating the stack into the heap later on so
const STACK_SIZE: usize = 16 * 8192 / size_of::<VMData>();

/// The stack for the VM
///
/// The stack is a fixed-size array of [VMData] with a maximum size of [STACK_SIZE]
///
/// The stack is used to store values and objects during the execution of the program
///
/// Each time a function is called, a new stack frame is created.
/// A stack frame contains the values of the function's arguments and local variables
/// as well as the temporary values used during the execution of the function.
///
/// A stack frame takes this form:
/// - `[[arguments], [local variables], [temporary values]]`
/// ``arguments`` & ``local variables`` are fixed-size arrays
#[derive(Debug)]
pub struct Stack<'stack> {
    values: [VMData<'stack>; STACK_SIZE],
    pub top: usize,
}
#[derive(Debug)]
pub struct StackFrameInfo {
    pub pc: usize,
    pub base_ptr: usize,
}
impl Default for Stack<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// TODO: this implementation should be overhauled a bit cuz it's kinda clunky
impl<'stack> Stack<'stack> {
    pub fn new() -> Self {
        Self {
            values: [VMData::new_unit(); STACK_SIZE],
            top: 0,
        }
    }
    pub fn clear(&mut self) {
        self.top = 0;
    }

    pub fn push(&mut self, val: VMData) -> Result<(), RuntimeError> {
        if self.top < STACK_SIZE {
            self.values[self.top] = val;
            self.top += 1;
            Ok(())
        } else {
            Self::push_stack_overflow()
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
            Self::push_stack_overflow()
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

    #[inline(always)]
    pub fn pop(&mut self) -> Result<VMData<'stack>, RuntimeError> {
        self.top = self.top.wrapping_sub(1); // Always decrement

        // If underflow happens, restore `self.top` and return an error.
        if self.top == usize::MAX {
            self.top = 0; // Restore previous state
            return Self::pop_stack_underflow();
        }
        Ok(self.values[self.top])
    }
    pub fn pop_with_rc(&mut self, mem: &mut Memory) -> Result<VMData<'stack>, RuntimeError> {
        self.top = self.top.wrapping_sub(1); // Always decrement

        if self.top == usize::MAX {
            self.top = 0;
            return Self::pop_stack_underflow();
        }

        let r = self.values[self.top];
        match r.tag {
            VMData::TAG_OBJECT | VMData::TAG_LIST | VMData::TAG_STR => {
                mem.rc_dec(r.as_object())?;
            }
            _ => {}
        }
        Ok(r)
    }

    #[cold]
    #[inline(never)]
    fn pop_stack_underflow() -> Result<VMData<'stack>, RuntimeError> {
        Err(RuntimeError::StackUnderflow)
    }

    #[cold]
    #[inline(never)]
    fn push_stack_overflow() -> Result<(), RuntimeError> {
        Err(RuntimeError::StackOverflow)
    }

    #[inline(always)]
    pub fn last(&self) -> Result<&VMData<'stack>, RuntimeError> {
        if self.top != 0 {
            Ok(&self.values[self.top - 1])
        } else {
            Err(RuntimeError::StackUnderflow)
        }
    }

    pub fn extends(&mut self, values: &[VMData<'stack>]) -> Result<(), RuntimeError> {
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

    pub fn push_object(&mut self, _obj: &[VMData<'stack>]) -> Result<(), RuntimeError> {
        unimplemented!("push_object(&mut self, obj: &[VMData])")
    }

    pub fn new_stack_frame(&mut self) {}

    pub fn set(&mut self, _offset: usize) {}

    pub fn iter(&self) -> std::slice::Iter<VMData> {
        self.values[..self.top].iter()
    }
}

impl<'stack> IntoIterator for Stack<'stack> {
    type Item = VMData<'stack>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values[..self.top].to_vec().into_iter()
    }
}

impl<'stack> Index<usize> for Stack<'stack> {
    type Output = VMData<'stack>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Stack<'_> {
    fn index_mut(&mut self, index: usize) -> &mut VMData {
        &mut self.values[index]
    }
}

impl<'stack> Index<StackFrameInfo> for Stack<'stack> {
    type Output = [VMData<'stack>];
    fn index(&self, index: StackFrameInfo) -> &Self::Output {
        &self.values[index.base_ptr..index.pc]
    }
}


impl Display for Stack<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stack: {}",
            {
                let mut s = "[".to_string();
                for i in 0..self.top {
                    s.push_str(&format!("{}, ", self.values[i]))
                }
                s.push(']');
                s
            },
        )
    }
}
