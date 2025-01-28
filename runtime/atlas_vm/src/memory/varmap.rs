use crate::memory::object_map::Memory;
use crate::memory::vm_data::VMData;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct VarMap {
    var_map: Vec<HashMap<String, VMData>>,
}

impl VarMap {
    pub fn new() -> Self {
        VarMap {
            var_map: vec![HashMap::new()]
        }
    }
    /// Insert doesn't need to increment the reference count of the value.
    /// Because stack.pop() doesn't decrement the reference count of the value.
    pub fn insert(&mut self, key: String, value: VMData, mem: &mut Memory) -> Option<VMData> {
        let old_data = self.var_map.last_mut().unwrap().insert(key, value);
        match old_data {
            Some(old_data) => {
                match old_data.tag {
                    VMData::TAG_STR | VMData::TAG_LIST | VMData::TAG_OBJECT => {
                        mem.rc_dec(old_data.as_object());
                    }
                    _ => {}
                }
            }
            None => {}
        }
        old_data
    }
    pub fn get(&self, key: &str) -> Option<&VMData> {
        self.var_map.last().unwrap().get(key)
    }
    pub fn last(&self) -> &HashMap<String, VMData> {
        self.var_map.last().unwrap()
    }
    pub fn push(&mut self) {
        self.var_map.push(HashMap::new());
    }
    pub fn pop(&mut self, mem: &mut Memory) {
        let map = self.var_map.pop().unwrap();
        for (_, value) in map {
            match value.tag {
                VMData::TAG_STR | VMData::TAG_LIST | VMData::TAG_OBJECT => {
                    mem.rc_dec(value.as_object());
                }
                _ => {}
            }
        }
    }
}