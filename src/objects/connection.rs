use crate::{CloneRef, Error, IntoObject, Object, Result};
use std::rc::Rc;

pub struct ConnectionConfig {
    pub id: String,
    pub source: Object,
    pub target: Object,
}

pub type ConnectionRef = Rc<ConnectionConfig>;

pub struct ConnectionBuilder {
    id: String,
    source: Option<Object>,
    target: Option<Object>,
}

impl ConnectionBuilder {
    pub fn new(id: String) -> ConnectionBuilder {
        ConnectionBuilder {
            id,
            source: None,
            target: None,
        }
    }

    pub fn from<T: CloneRef<T> + IntoObject>(&mut self, resource: T) -> &mut Self {
        self.source = Some(resource.clone_ref().into_object());
        self
    }

    pub fn to<T: CloneRef<T> + IntoObject>(&mut self, resource: T) -> &mut Self {
        self.target = Some(resource.clone_ref().into_object());
        self
    }

    pub fn build(&self) -> Result<ConnectionRef> {
        let config = ConnectionConfig {
            id: self.id.clone(),
            source: match &self.source {
                Some(s) => s.clone(),
                None => {
                    return Err(Error::new(format!(
                        "missing source in connection {}",
                        self.id
                    )))
                }
            },
            target: match &self.target {
                Some(s) => s.clone(),
                None => {
                    return Err(Error::new(format!(
                        "missing target in connection {}",
                        self.id
                    )))
                }
            },
        };

        Ok(ConnectionRef::new(config))
    }
}

impl CloneRef<ConnectionRef> for ConnectionRef {
    fn clone_ref(&self) -> ConnectionRef {
        ConnectionRef::clone(&self)
    }
}

impl IntoObject for ConnectionRef {
    fn into_object(self) -> Object {
        Object::Connection(self)
    }
}
