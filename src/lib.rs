#[warn(missing_docs)]
//At one point, every part of the language will be in its own crate.
//Or at least separated between the frontend, the backend, the runtime and the standard library.

/// WIP: Codegen
pub mod atlas_codegen;
/// The frontend is responsible for parsing the code and generating the HIR as well as multiple passes to check for errors.
pub mod atlas_frontend;
/// The HIR is the High-level Intermediate Representation.
pub mod atlas_hir;
/// The macro crate is where all the macros used by the language are defined.
pub mod atlas_macro;
/// Contains the memory model for the VM
pub mod atlas_memory;
/// The runtime will soon be deprecated in favor of the VM
///
/// It'll still be used for compile time evaluation (but will need to be reworked)
pub mod atlas_runtime;
/// The standard library
pub mod atlas_stdlib;
/// The Virtual Machine that will run the code.
pub mod atlas_vm;
