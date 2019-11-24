mod error;
mod objects;

pub use error::{Error, Result};
pub use objects::compute::{ComputeBuilder, ComputeRef};
pub use objects::connection::{ConnectionBuilder, ConnectionRef};
pub use objects::project::Project;
pub use objects::storage::{StorageBuilder, StorageRef};

pub enum Object {
    Compute(ComputeRef),
    Storage(StorageRef),
    Connection(ConnectionRef),
}

impl Clone for Object {
    fn clone(&self) -> Object {
        match self {
            Object::Compute(r) => Object::Compute(r.clone_ref()),
            Object::Storage(r) => Object::Storage(r.clone_ref()),
            Object::Connection(r) => Object::Connection(r.clone_ref()),
        }
    }
}

pub trait Deploy {
    fn deploy_compute(&self, config: ComputeRef) -> Result<()>;
    fn deploy_storage(&self, config: StorageRef) -> Result<()>;
    fn deploy_connection(&self, config: ConnectionRef) -> Result<()>;
}

pub trait CloneRef<T> {
    fn clone_ref(&self) -> T;
}

pub trait IntoObject {
    fn into_object(self) -> Object;
}

pub trait LockFileSerialize {
    fn lock_file_serialize(&self) -> String;
}

pub struct LockFile<T>
where
    T: LockFileSerialize,
{
    content: T,
}
