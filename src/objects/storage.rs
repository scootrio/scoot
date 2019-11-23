use crate::{AsObj, ScootObject};
use std::rc::Rc;

pub struct StorageConfig {
    pub id: String,
}

impl StorageConfig {
    fn new(id: String) -> StorageConfig {
        StorageConfig { id }
    }
}

pub type StorageConfigRef = Rc<StorageConfig>;

pub struct StorageBuilder {
    config: StorageConfigRef,
}

impl StorageBuilder {
    pub fn new(id: String) -> StorageBuilder {
        StorageBuilder {
            config: Rc::new(StorageConfig::new(id)),
        }
    }
}

impl AsObj for StorageBuilder {
    fn as_obj(&self) -> ScootObject {
        ScootObject::Storage(StorageConfigRef::clone(&self.config))
    }
}
