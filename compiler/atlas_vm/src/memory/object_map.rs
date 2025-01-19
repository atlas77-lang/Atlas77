use crate::memory::vm_data::VMData;

/// Probably should be renamed lmao
///
/// Need to find a way to make the memory shrink, grows, and garbage collect unused memory (by scanning the stack & VarMap)
pub struct Memory {
    mem: Vec<Object>,
    pub free: ObjectIndex,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ObjectIndex {
    pub idx: u64,
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
                .map(|x| Object::Free {
                    next: ObjectIndex::new(((x + 1) % space) as u64),
                })
                .collect(),
        }
    }

    /// Need to add a way to increase `mem` size if we out of memory
    /// And a way to clean it when there's too much memory (basically shrink and grow)
    ///
    /// Result<ObjectIndex, Object> should become Result<ObjectIndex, RuntimeError> with RuntimeError::OutOfMemory(Object)
    pub fn put(&mut self, object: Object) -> Result<ObjectIndex, Object> {
        let idx = self.free;
        let v = self.get_mut(self.free);
        let repl = std::mem::replace(v, object);

        match repl {
            Object::Free { next } => {
                self.free = next;
                Ok(idx)
            }
            _ => {
                let obj = std::mem::replace(v, repl);
                Err(obj)
            }
        }
    }

    #[inline(always)]
    pub fn get(&self, index: ObjectIndex) -> &Object {
        &self.mem[index.idx as usize]
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: ObjectIndex) -> &mut Object {
        &mut self.mem[index.idx as usize]
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

/// List is imho badly represented, a list should be defined in the language itself as a wrapper around a static array
///
#[derive(Clone, Debug)]
pub enum Object {
    String(String),
    Structure(Structure),
    List(Vec<VMData>),
    Free { next: ObjectIndex },
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Structure(s) => write!(f, "{:?}", s),
            Object::List(l) => write!(f, "{:?}", l),
            Object::Free { next } => write!(f, "Free: next -> {}", next),
        }
    }
}

impl Object {
    pub fn new(data: impl Into<Object>) -> Self {
        data.into()
    }

    pub fn string(&self) -> &String {
        match &self {
            Object::String(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn string_mut(&mut self) -> &mut String {
        match self {
            Object::String(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn structure(&self) -> &Structure {
        match &self {
            Object::Structure(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn structure_mut(&mut self) -> &mut Structure {
        match self {
            Object::Structure(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn list(&self) -> &Vec<VMData> {
        match &self {
            Object::List(l) => l,
            _ => unreachable!(),
        }
    }

    pub fn list_mut(&mut self) -> &mut Vec<VMData> {
        match self {
            Object::List(l) => l,
            _ => unreachable!(),
        }
    }
}

impl From<Structure> for Object {
    fn from(value: Structure) -> Self {
        Object::Structure(value)
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::String(value)
    }
}

impl From<Vec<VMData>> for Object {
    fn from(value: Vec<VMData>) -> Self {
        Object::List(value)
    }
}

#[derive(Clone, Debug)]
pub struct Structure {
    pub fields: Vec<VMData>,
}
