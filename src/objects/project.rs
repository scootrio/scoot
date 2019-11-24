use crate::{Deploy, Error, IntoObject, Object, Result};

pub struct Project<T>
where
    T: Deploy,
{
    objects: Vec<Object>,
    driver: Option<T>,
}

impl<T> Project<T>
where
    T: Deploy,
{
    pub fn new() -> Project<T> {
        Project {
            objects: Vec::new(),
            driver: None,
        }
    }
    pub fn with<R: IntoObject>(mut self, resource: R) -> Self {
        self.objects.push(resource.into_object());
        self
    }
    pub fn with_driver(mut self, driver: T) -> Self {
        self.driver = Some(driver);
        self
    }
    pub fn deploy(self) -> Result<()> {
        match self.driver {
            None => return Err(Error::new("missing driver for scoot deployment")),
            Some(driver) => {
                for object in self.objects {
                    match object {
                        Object::Compute(config) => driver.deploy_compute(config)?,
                        Object::Storage(config) => driver.deploy_storage(config)?,
                        Object::Connection(config) => driver.deploy_connection(config)?,
                    }
                }
                Ok(())
            }
        }
    }
}
