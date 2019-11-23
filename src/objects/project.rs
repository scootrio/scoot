use crate::{AsObj, Deploy, ScootObject};

pub struct Project {
    objects: Vec<ScootObject>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            objects: Vec::new(),
        }
    }
    pub fn with(mut self, obj: impl AsObj) -> Self {
        self.objects.push(obj.as_obj());
        self
    }
    pub fn deploy_with(self, driver: impl Deploy) -> Result<(), &'static str> {
        for object in self.objects {
            match object {
                ScootObject::Compute(config) => driver.deploy_compute(&*config),
                ScootObject::Storage(config) => driver.deploy_storage(&*config),
                ScootObject::Connection(config) => driver.deploy_connection(&*config),
            }
        }

        Ok(())
    }
}
