//! Native Code Property Graph schema. Implementation milestone: **UNVERIFIED**.
#![forbid(unsafe_code)]
#![allow(
    non_snake_case,
    reason = "Scala-compatible public schema module and member names"
)]
pub mod Annotation;
pub mod Ast;
pub mod Base;
pub mod Binding;
pub mod CallGraph;
pub mod Cfg;
pub mod Comment;
pub mod Configuration;
pub mod CpgSchema;
pub mod Dominators;
pub mod FileSystem;
pub mod Finding;
pub mod Hidden;
pub mod MetaData;
pub mod Method;
pub mod Namespace;
pub mod Operators;
pub mod Pdg;
pub mod ProtoIds;
pub mod ProtoSerialize;
pub mod Protogen;
pub mod SchemaBase;
pub mod Shortcuts;
pub mod Tags;
pub mod Type;
#[cfg(feature = "json-io")]
pub mod codec;
pub mod components;
pub mod graph;
pub mod model;
pub mod value;
pub use model::{Error, Result, Schema, SchemaBuilder};
pub const SOURCE_REVISION: &str = "d02d76ea5c9e3b398317d334228d7e8e78ab45aa";
pub const FLATGRAPH_VERSION: &str = "0.1.36";
