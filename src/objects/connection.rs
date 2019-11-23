use crate::{AsObj, ScootObject};
use std::rc::Rc;

pub struct ConnectionConfig {
    pub id: String,
    pub source: Option<ScootObject>,
    pub target: Option<ScootObject>,
}

impl ConnectionConfig {
    fn new(id: String) -> ConnectionConfig {
        ConnectionConfig {
            id,
            source: None,
            target: None,
        }
    }
}

pub type ConnectionConfigRef = Rc<ConnectionConfig>;

pub struct ConnectionBuilder {
    config: ConnectionConfigRef,
}

impl ConnectionBuilder {
    pub fn new(id: String) -> ConnectionBuilder {
        ConnectionBuilder {
            config: Rc::new(ConnectionConfig::new(id)),
        }
    }

    pub fn from(&mut self, obj: impl AsObj) -> &mut Self {
        if let Some(config) = ConnectionConfigRef::get_mut(&mut self.config) {
            config.source = Some(obj.as_obj());
        }
        self
    }

    pub fn to(&mut self, obj: impl AsObj) -> &mut Self {
        if let Some(config) = ConnectionConfigRef::get_mut(&mut self.config) {
            config.target = Some(obj.as_obj());
        }
        self
    }
}

impl AsObj for ConnectionBuilder {
    fn as_obj(&self) -> ScootObject {
        ScootObject::Connection(ConnectionConfigRef::clone(&self.config))
    }
}
