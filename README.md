# Scoot
Scoot is a library consisting of a fluent API for representing the software architecture of a cloud-based system in Rust. The purpose of the library is twofold:

1. facilitate the representation of architecture of a software system as code and 
2. provision and configure the infrastructure required to complete the architecture when definitions are provided to a driver.

## Example Usage

```rs
use scoot;

fn main() {

    let compute = scoot::ComputeBuilder::new("my-compute")
        .vcs("https://github.com/example/code")
        .build();

    let storage = scoot::StorageBuilder::new("my-storage").build();

    let connection = scoot::ConnectionBuilder::new("my-connection")
        .from(&compute)
        .to(&storage)
        .build();

    let project = scoot::Project::new();
        .with(compute)
        .with(storage)
        .with(connection);

}
```