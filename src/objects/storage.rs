use crate::{CloneRef, IntoObject, Object, Result};
use std::rc::Rc;

pub struct StorageConfig {
    pub id: String,
}

pub type StorageRef = Rc<StorageConfig>;

pub struct StorageBuilder {
    id: String,
}

impl StorageBuilder {
    pub fn new(id: String) -> StorageBuilder {
        StorageBuilder { id }
    }

    pub fn build(&self) -> Result<StorageRef> {
        let config = StorageConfig {
            id: self.id.clone(),
        };

        Ok(StorageRef::new(config))
    }
}

impl CloneRef<StorageRef> for StorageRef {
    fn clone_ref(&self) -> StorageRef {
        StorageRef::clone(&self)
    }
}

impl IntoObject for StorageRef {
    fn into_object(self) -> Object {
        Object::Storage(self)
    }
}
