use crate::{LyssRuntimeError, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ObjectEntry<V> {
    Branch(Object<V>),
    Leaf(V),
}

pub enum ObjectSearch<'o, V> {
    Branch(&'o Object<V>),
    Leaf(&'o V),
}

#[derive(Debug, Clone)]
pub struct Object<V>(pub HashMap<String, ObjectEntry<V>>);

/// The `$` Object
#[derive(Debug, Default, Clone)]
pub struct VarsObject(pub Object<Value>);

#[derive(Debug, Clone)]
enum ObjectFindResult {
    NotFound,
    EarlyLeaf,
}

impl<V: Clone> Object<V> {
    pub fn find_leaf(&self, path: &[String]) -> Result<V, LyssRuntimeError> {
        let obj_entry = self.find_deep(path.iter()).map_err(|e| match e {
            ObjectFindResult::EarlyLeaf => LyssRuntimeError::EntryWasLeaf {
                path: path.to_vec(),
            },
            ObjectFindResult::NotFound => LyssRuntimeError::EntryNotFound {
                path: path.to_vec(),
            },
        })?;
        match obj_entry {
            ObjectSearch::Leaf(l) => Ok(l.clone()),
            ObjectSearch::Branch(_) => Err(LyssRuntimeError::EntryWasBranch {
                path: path.to_vec(),
            }),
        }
    }
    pub fn find_branch(&self, path: &[String]) -> Result<&Object<V>, LyssRuntimeError> {
        let obj_entry = self.find_deep(path.iter()).map_err(|e| match e {
            ObjectFindResult::EarlyLeaf => LyssRuntimeError::EntryWasLeaf {
                path: path.to_vec(),
            },
            ObjectFindResult::NotFound => LyssRuntimeError::EntryNotFound {
                path: path.to_vec(),
            },
        })?;
        match obj_entry {
            ObjectSearch::Leaf(_) => todo!(),
            ObjectSearch::Branch(b) => Ok(b),
        }
    }
    fn find_deep(
        &self,
        paths: std::slice::Iter<String>,
    ) -> Result<ObjectSearch<'_, V>, ObjectFindResult> {
        let mut obj = ObjectSearch::Branch(self);
        for path in paths {
            obj = match obj {
                ObjectSearch::Branch(b) => b.find_next(path).ok_or(ObjectFindResult::NotFound),
                ObjectSearch::Leaf(_) => Err(ObjectFindResult::EarlyLeaf),
            }?;
        }
        Ok(obj)
    }
    fn find_next(&self, path: &str) -> Option<ObjectSearch<'_, V>> {
        Some(match self.0.get(path)? {
            ObjectEntry::Leaf(l) => ObjectSearch::Leaf(l),
            ObjectEntry::Branch(b) => ObjectSearch::Branch(b),
        })
    }
}

impl<V: Clone> Default for Object<V> {
    fn default() -> Self {
        Object(HashMap::default())
    }
}
