use crate::atlas_vm::errors::RuntimeError;
use crate::atlas_vm::memory::vm_data::VMData;
use crate::atlas_vm::RuntimeResult;

/// Probably should be renamed lmao
///
/// Need to find a way to make the memory shrink, grows, and garbage collect unused memory (by scanning the stack & VarMap)
pub struct Memory {
    mem: Vec<Object>,
    pub free: ObjectIndex,
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

impl Memory {
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
        }
    }

    pub fn put(&mut self, object: ObjectKind) -> Result<ObjectIndex, RuntimeError> {
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
        //println!("Freeing object: {}", index);
        let next = self.free;
        let v = self.mem.get(usize::from(index)).unwrap().kind.clone();
        match v {
            ObjectKind::List(l) => {
                for item in l {
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
        //println!("Getting object: {} {}", kind, index);
        self.rc_dec(index)?;
        Ok(kind)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: ObjectIndex) -> RuntimeResult<&mut ObjectKind> {
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
    pub fn raw_mut(&mut self) -> &mut [Object] {
        &mut self.mem
    }
}

impl std::fmt::Display for Memory {
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

#[derive(Clone, Debug)]
pub enum ObjectKind {
    String(String),
    Structure(Structure),
    List(Vec<VMData>),
    Free { next: ObjectIndex },
}
impl Default for ObjectKind {
    fn default() -> Self {
        ObjectKind::Free {
            next: ObjectIndex::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub kind: ObjectKind,
    /// Reference count
    pub rc: usize,
}
impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (rc: {})", self.kind, self.rc)
    }
}

impl std::fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKind::String(s) => write!(f, "`String`: \"{}\"", s),
            ObjectKind::Structure(s) => write!(f, "{:?}", s),
            ObjectKind::List(l) => write!(f, "{:?}", l),
            ObjectKind::Free { next } => write!(f, "Free: next -> {}", next),
        }
    }
}

impl ObjectKind {
    pub fn new(data: impl Into<ObjectKind>) -> Self {
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

    pub fn structure(&self) -> &Structure {
        match &self {
            ObjectKind::Structure(s) => s,
            _ => unreachable!("Expected a structure, got a {:?}", self),
        }
    }

    pub fn structure_mut(&mut self) -> &mut Structure {
        match self {
            ObjectKind::Structure(s) => s,
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

impl From<Structure> for ObjectKind {
    fn from(value: Structure) -> Self {
        ObjectKind::Structure(value)
    }
}

impl From<String> for ObjectKind {
    fn from(value: String) -> Self {
        ObjectKind::String(value)
    }
}

impl From<Vec<VMData>> for ObjectKind {
    fn from(value: Vec<VMData>) -> Self {
        ObjectKind::List(value)
    }
}

#[derive(Clone, Debug)]
pub struct Structure {
    pub fields: Vec<VMData>,
}
