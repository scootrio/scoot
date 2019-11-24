use crate::{CloneRef, Error, IntoObject, Object, Result};
use std::rc::Rc;
use url;

pub struct ComputeConfig {
    pub id: String,
    pub vcs: url::Url,
}

pub type ComputeRef = Rc<ComputeConfig>;

#[derive(Default)]
pub struct ComputeBuilder {
    id: String,
    vcs: Option<String>,
}

impl ComputeBuilder {
    pub fn new(id: String) -> ComputeBuilder {
        ComputeBuilder {
            id,
            ..Default::default()
        }
    }

    pub fn vcs(&mut self, vcs: String) -> &mut Self {
        self.vcs = Some(vcs);
        self
    }

    pub fn build(&self) -> Result<ComputeRef> {
        let config = ComputeConfig {
            id: self.id.clone(),
            vcs: if let Some(val) = self.vcs.clone() {
                match url::Url::parse(&val) {
                    Ok(url) => url,
                    Err(e) => {
                        return Err(Error::new(format!(
                            "failed to parse URL for source code control: {}",
                            e
                        )))
                    }
                }
            } else {
                return Err(Error::new("missing URL for source code control"));
            },
        };

        Ok(ComputeRef::new(config))
    }
}

impl CloneRef<ComputeRef> for ComputeRef {
    fn clone_ref(&self) -> ComputeRef {
        ComputeRef::clone(&self)
    }
}

impl IntoObject for ComputeRef {
    fn into_object(self) -> Object {
        Object::Compute(self)
    }
}
