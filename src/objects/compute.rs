use crate::{AsObj, ScootObject};
use std::rc::Rc;

pub struct ComputeConfig {
    pub id: String,
}

impl ComputeConfig {
    fn new(id: String) -> ComputeConfig {
        ComputeConfig { id }
    }
}

pub type ComputeConfigRef = Rc<ComputeConfig>;

pub struct ComputeBuilder {
    config: ComputeConfigRef,
}

impl ComputeBuilder {
    pub fn new(id: String) -> ComputeBuilder {
        ComputeBuilder {
            config: Rc::new(ComputeConfig::new(id)),
        }
    }
}

impl AsObj for ComputeBuilder {
    fn as_obj(&self) -> ScootObject {
        ScootObject::Compute(ComputeConfigRef::clone(&self.config))
    }
}
