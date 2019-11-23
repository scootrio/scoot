mod objects;

pub use objects::compute::ComputeBuilder;
pub use objects::connection::ConnectionBuilder;
pub use objects::project::Project;
pub use objects::storage::StorageBuilder;

use objects::compute::{ComputeConfig, ComputeConfigRef};
use objects::connection::{ConnectionConfig, ConnectionConfigRef};
use objects::storage::{StorageConfig, StorageConfigRef};

pub enum ScootObject {
    Compute(ComputeConfigRef),
    Storage(StorageConfigRef),
    Connection(ConnectionConfigRef),
}

pub trait Deploy {
    fn deploy_compute(&self, config: &ComputeConfig);
    fn deploy_storage(&self, config: &StorageConfig);
    fn deploy_connection(&self, config: &ConnectionConfig);
}

pub trait AsObj {
    fn as_obj(&self) -> ScootObject;
}
