use crate::atlas_vm::errors::RuntimeError;
use crate::atlas_vm::memory::vm_data::VMData;
use crate::atlas_vm::RuntimeResult;
use std::collections::HashMap;

/// Probably should be renamed lmao
///
/// Need to find a way to make the memory shrink, grows, and garbage collect unused memory (by scanning the stack & VarMap)
pub struct Memory<'mem> {
    mem: Vec<Object<'mem>>,
    pub free: ObjectIndex,
    pub used_space: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ObjectIndex {
    pub idx: u64,
}

impl From<ObjectIndex> for usize {
    fn from(value: ObjectIndex) -> Self {
        value.idx as usize
    }
}

impl ObjectIndex {
    pub const fn new(i: u64) -> ObjectIndex {
        ObjectIndex { idx: i }
    }
}
impl std::fmt::Display for ObjectIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[@{}]", self.idx)
    }
}

impl<'mem> Memory<'mem> {
    pub fn new(space: usize) -> Self {
        Self {
            free: ObjectIndex::new(0),
            mem: (0..space)
                .map(|x| Object {
                    kind: ObjectKind::Free {
                        next: ObjectIndex::new(((x + 1) % space) as u64),
                    },
                    rc: 0,
                })
                .collect(),
            used_space: 0,
        }
    }
    pub fn clear(&mut self) {
        for (idx, obj) in self.mem.iter_mut().enumerate() {
            obj.kind = ObjectKind::Free {
                next: self.free,
            };
            obj.rc = 0;
            self.free = ObjectIndex::new(idx as u64);
        }
    }

    pub fn put(&mut self, object: ObjectKind<'mem>) -> Result<ObjectIndex, RuntimeError> {
        if self.used_space == self.mem.len() {
            for i in self.mem.len()..(self.mem.len() * 2) {
                self.mem.push(Object {
                    kind: ObjectKind::Free {
                        next: self.free,
                    },
                    rc: 0,
                });
                self.free = ObjectIndex::new(i as u64);
            }
        }
        let idx = self.free;
        let v = self.mem.get_mut(usize::from(self.free)).unwrap();
        let repl = std::mem::replace(v, Object { kind: object, rc: 1 });

        match repl {
            Object { kind: ObjectKind::Free { next }, .. } => {
                self.free = next;
                Ok(idx)
            }
            _ => {
                Err(RuntimeError::OutOfMemory)
            }
        }
    }

    pub fn free(&mut self, index: ObjectIndex) -> RuntimeResult<()> {
        let next = self.free;
        let v = self.mem.get_mut(usize::from(index)).unwrap().kind.clone();
        match v {
            ObjectKind::List(list) => {
                for item in list {
                    match item.tag {
                        VMData::TAG_STR | VMData::TAG_LIST | VMData::TAG_OBJECT => {
                            let obj_to_dec = item.as_object();
                            self.rc_dec(obj_to_dec)?;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        let v = self.mem.get_mut(usize::from(index)).unwrap();
        let repl = std::mem::replace(
            v,
            Object {
                kind: ObjectKind::Free { next },
                rc: 0,
            },
        );
        let res = match repl {
            Object { kind: ObjectKind::Free { .. }, .. } => {
                Err(RuntimeError::NullReference)
            }
            _ => {
                Ok(())
            }
        };
        self.free = index;
        res
    }

    #[inline(always)]
    pub fn get(&mut self, index: ObjectIndex) -> RuntimeResult<ObjectKind> {
        let kind = self.mem[usize::from(index)].kind.clone();
        self.rc_dec(index)?;
        Ok(kind)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: ObjectIndex) -> RuntimeResult<&mut ObjectKind<'mem>> {
        //You can decrement the rc here, because if it reaches 0 and still need to return a mutable reference, it's a bug
        self.rc_dec(index)?;
        let kind = &mut self.mem[usize::from(index)].kind;
        Ok(kind)
    }

    #[inline(always)]
    pub fn rc_inc(&mut self, index: ObjectIndex) {
        self.mem[usize::from(index)].rc += 1;
    }

    #[inline(always)]
    pub fn rc_dec(&mut self, index: ObjectIndex) -> RuntimeResult<()> {
        let rc = &mut self.mem[usize::from(index)].rc;
        *rc -= 1;
        if *rc == 0 {
            self.free(index)?;
        }
        Ok(())
    }

    #[inline(always)]
    pub fn raw(&self) -> &[Object] {
        &self.mem
    }

    #[inline(always)]
    pub fn raw_mut(&mut self) -> &mut [Object<'mem>] {
        &mut self.mem
    }
}

impl std::fmt::Display for Memory<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, obj) in self.mem.iter().enumerate() {
            if let Object { kind: ObjectKind::Free { .. }, .. } = obj {
                continue;
            }
            writeln!(f, "{}: {}", i, obj)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ObjectKind<'mem> {
    String(String),
    Class(Class<'mem>),
    List(Vec<VMData>),
    Free { next: ObjectIndex },
}
impl Default for ObjectKind<'_> {
    fn default() -> Self {
        ObjectKind::Free {
            next: ObjectIndex::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Object<'mem> {
    pub kind: ObjectKind<'mem>,
    /// Reference count
    pub rc: usize,
}
impl std::fmt::Display for Object<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (rc: {})", self.kind, self.rc)
    }
}

impl std::fmt::Display for ObjectKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKind::String(s) => write!(f, "`String`: \"{}\"", s),
            ObjectKind::Class(s) => write!(f, "{:?}", s),
            ObjectKind::List(l) => write!(f, "{:?}", l),
            ObjectKind::Free { next } => write!(f, "Free: next -> {}", next),
        }
    }
}

impl<'mem> ObjectKind<'mem> {
    pub fn new(data: impl Into<ObjectKind<'mem>>) -> Self {
        data.into()
    }

    pub fn string(&self) -> &String {
        match &self {
            ObjectKind::String(s) => s,
            _ => unreachable!("Expected a string, got a {:?}", self),
        }
    }

    pub fn string_mut(&mut self) -> &mut String {
        match self {
            ObjectKind::String(s) => s,
            _ => unreachable!("Expected a string, got a {:?}", self),
        }
    }

    pub fn class(&self) -> &Class<'mem> {
        match &self {
            ObjectKind::Class(s) => s,
            _ => unreachable!("Expected a structure, got a {:?}", self),
        }
    }

    pub fn class_mut(&mut self) -> &mut Class<'mem> {
        match self {
            ObjectKind::Class(s) => s,
            _ => unreachable!("Expected a structure, got a {:?}", self),
        }
    }

    pub fn list(&self) -> &Vec<VMData> {
        match &self {
            ObjectKind::List(l) => l,
            _ => unreachable!("Expected a list, got a {:?}", self),
        }
    }

    pub fn list_mut(&mut self) -> &mut Vec<VMData> {
        match self {
            ObjectKind::List(l) => l,
            _ => unreachable!("Expected a list, got a {:?}", self),
        }
    }
}


impl<'mem> From<Class<'mem>> for ObjectKind<'mem> {
    fn from(value: Class<'mem>) -> Self {
        ObjectKind::Class(value)
    }
}

impl<'mem> From<String> for ObjectKind<'mem> {
    fn from(value: String) -> Self {
        ObjectKind::String(value)
    }
}

impl<'mem> From<Vec<VMData>> for ObjectKind<'mem> {
    fn from(value: Vec<VMData>) -> Self {
        ObjectKind::List(value)
    }
}

#[derive(Clone, Debug)]
pub struct Class<'mem> {
    pub fields: HashMap<&'mem str, VMData>,
}
