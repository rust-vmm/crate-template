//! Helper macros to opt-in/opt-out serde.
//!
//! A common requirement of VM snapshot, VM live upgrading and VM live migration is to serialize and
//! deserialize VMM internal objects and states. This crate provides helper macros to help serialize
//! and deserialize VMM internal objects and states.
//!
//! # Procedural Macro to Export Private Fields of Struct as Public
//! To hide crate/module implementation details from the crate/module users, it's suggested to mark
//! all internal fields as private. But when enabling the VM snapshot/live upgrading/live migration,
//! the snapshot/live upgrading/live migration subsystem often needs to access other crate/module's
//! internal state. In other words, the snapshot/live upgrading/live migration subsystem needs to
//! access struct fields normally marked as private. So the export_as_pub proc macro is introduced
//! to mark struct fields as pub when the `export_as_pub` feature is enabled. Otherwise the
//! export_as_pub proc macro translates to a noop.
//!
//! ## Example
//! Suppose we have a Struct defined as:
//! ```
//! # extern crate vmm_serde;
//!
//! #[vmm_serde::export_as_pub()]
//! pub(crate) struct VmmObject {
//!     state: u32,
//!     pub(crate) features: u64,
//! }
//! ```
//!
//! When the `export_as_pub` feature is enabled, the Struct will be translated as:
//! ```
//! #[vmm_serde::export_as_pub()]
//! pub struct VmmObject {
//!     pub state: u32,
//!     pub features: u64,
//! }
//! ```
//!
//! And when the `export_as_pub` feature is disabled, the Struct will be translated as:
//! ```
//! pub(crate) struct VmmObject {
//!     state: u32,
//!     pub(crate) features: u64,
//! }
//! ```
//!
//! Instead of exporting all fields as pub, user may specify the fields needed to be pub as:
//! ```
//! # extern crate vmm_serde;
//!
//! #[vmm_serde::export_as_pub(features)]
//! pub(crate) struct VmmObject {
//!     state: u32,
//!     pub(crate) features: u64,
//! }
//! ```
//!
//! # Use Feature to Control #[derive(Serialize, Deserialize)]
//! The serde_derive crate exports proc_macro_derive(Serialize, Deserialize) to support the serde
//! crate, but it does introduce heavy dependency chains. So introduce the feature `serde_derive`.
//! When the feature `serde_derive` is enabled, implementation of #[derive(Serialize, Deserialize)]
//! is reexported from the serde_derive crate. When the feature `serde_derive` is disabled, a noop
//! implementation for  #[derive(Serialize, Deserialize)] is provided.
//!
//! ## Example
//! ```
//! # extern crate vmm_serde;
//! # use vmm_serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! pub struct VmmObject {
//!     state: u32,
//! }
//! ```

#[cfg(feature = "serde_derive")]
#[doc(hidden)]
pub use serde::*;

#[doc(hidden)]
pub use vmm_serde_impl::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(dead_code)]
    fn test_export_as_pub() {
        #[allow(unused_imports)]
        #[export_as_pub()]
        use std::result::Result;

        #[export_as_pub()]
        pub(super) struct VmmObject1 {
            state: u32,
            pub(crate) features: u64,
        }

        #[export_as_pub(state)]
        pub struct VmmObject2 {
            state: u32,
            pub(crate) features: u64,
        }

        #[export_as_pub(features)]
        struct VmmObject3 {
            state: u32,
            pub(crate) features: u64,
        }

        #[export_as_pub(state, features)]
        struct VmmObject4 {
            state: u32,
            pub(crate) features: u64,
        }
    }

    #[test]
    #[allow(dead_code)]
    fn test_derive() {
        #[derive(Serialize, Deserialize)]
        pub(super) struct VmmObject5 {
            state: u32,
        }

        #[derive(Serialize)]
        pub(super) struct VmmObject6 {
            state: u32,
        }

        #[derive(Deserialize)]
        pub(super) struct VmmObject7 {
            state: u32,
        }
    }
}
