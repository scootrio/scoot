# Scoot
Scoot is a library consisting of a fluent API for representing the software architecture of a cloud-based system in Rust. The purpose of the library is twofold:

1. facilitate the representation of architecture of a software system as code and 
2. provision and configure the infrastructure required to complete the architecture when definitions are provided to a driver.

## Example Usage

```rs
use scoot;

fn main() {

    let compute = scoot::ComputeBuilder::new("my-compute");

    let storage = scoot::StorageBuilder::new("my-storage");

    let connection = scoot::ConnectionBuilder::new("my-connection");
    connection.from(&compute).to(&storage);

    let project = scoot::Project::new();
    project.with_compute(compute).with_storage(storage).with_connection(connection);

}
```