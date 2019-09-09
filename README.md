# vmm-serde

## Design

A common requirement for virtual machine snapshot, live migration and live upgrading is to
serialize and deserialize VMM internal objects and data structures. The rust community has 
the excellent serde/serde_derive framework for data serialization/deserialization. But the
serde/serde_derive framework have very heavy dependency chain, and it's a common concern of the
rust-vmm project to reduce dependency chain as small as possible. So this helper crate uses
features to opt-in data serialization/deserialization functionality.

Currently there are two features defined:
- Feature `export_as_pub`
The data serializer often needs to access private data structures or private fields in data
structures. But rust has no support for `friend` visibility yet, though there's proposals for 
C++ like `friend` relationship. So a proc macro `export_as_pub()` is introduced to rewrite private
data structs/fields as `pub` when the feature `export_as_pub` is enabled. Otherwise
`export_as_pub()` becomes a null operation.

- Feature `serde_derive`
When the `serde_derive` feature is enabled, the proc_macro_derive for Serialize/Deserialize is
reexported from the serde_derive crate, otherwise `#[derive(Serialize, Deserialize)]` becomes
a null operation.

## Examples

### Export all fields of a data struct as `pub`
The `struct VmmObject`, field `state` and field `features` will be rewritten as `pub` when the
`export_as_pub` feature is enabled.
```rust
extern crate vmm_serde;

#[vmm_serde::export_as_pub()]
pub(crate) struct VmmObject {
    state: u32,
    pub(crate) features: u64,
}
```

### Export selected fields of a data struct as `pub`
The `struct VmmObject` and field `features` will be rewritten as `pub` when the
`export_as_pub` feature is enabled.
```
extern crate vmm_serde;

[vmm_serde::export_as_pub(features)]
pub(crate) struct VmmObject {
    state: u32,
    pub(crate) features: u64,
}
```

### Opt-in/Opt-out \#[derive(Serialize, Deserialize)]
Use feature `serde_derive` to opt-in/opt-out Serialize/Deserialize
```
extern crate vmm_serde;
use vmm_serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct VmmObject {
    state: u32,
}
```

## License
This project is licensed under

- Apache License, Version 2.0
- BSD 3 Clause

