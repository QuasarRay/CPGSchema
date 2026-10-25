//! Project-wide CpgSchema interfaces.
//!
//! This module is deliberately independent of JSON Schema, serde, and the
//! native representation. Backends expose their semantics through these traits.

use std::{collections::BTreeSet, fmt};

/// One schema module in construction order.
pub trait SchemaModuleContract {
    /// Stable module name, e.g. `Base`, `Method`, or `Ast`.
    fn name(&self) -> &str;

    /// Direct module dependencies, in source order.
    fn dependencies(&self) -> &[&str];
}

/// Common property-default contract from `CpgSchema.PropertyDefaults`.
pub trait PropertyDefaultsContract {
    fn string_default(&self) -> &str;
    fn boolean_default(&self) -> bool;
    fn byte_default(&self) -> i8;
    fn short_default(&self) -> i16;
    fn int_default(&self) -> i32;
    fn long_default(&self) -> i64;
    fn float_default(&self) -> f32;
    fn double_default(&self) -> f64;
    fn char_default(&self) -> char;
}

/// Common interface for the complete CPG schema composition.
///
/// Both the native Rust implementation and the optional JSON-backed adapter
/// implement this interface.
pub trait CpgSchemaContract {
    type Module: SchemaModuleContract;
    type Defaults: PropertyDefaultsContract;

    fn domain_short_name(&self) -> &str;
    fn base_package(&self) -> &str;
    fn additional_traversals_packages(&self) -> &[&str];
    fn modules(&self) -> &[Self::Module];
    fn property_defaults(&self) -> &Self::Defaults;
}

/// Representation-independent contract error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError {
    EmptyDomainShortName,
    EmptyBasePackage,
    EmptyTraversalPackage { index: usize },
    EmptyModuleName { index: usize },
    DuplicateModule { name: String },
    DuplicateDependency { module: String, dependency: String },
    MissingOrForwardDependency { module: String, dependency: String },
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDomainShortName => f.write_str("domain short name must not be empty"),
            Self::EmptyBasePackage => f.write_str("base package must not be empty"),
            Self::EmptyTraversalPackage { index } => {
                write!(f, "additional traversal package {index} must not be empty")
            }
            Self::EmptyModuleName { index } => {
                write!(f, "module name at index {index} must not be empty")
            }
            Self::DuplicateModule { name } => write!(f, "module `{name}` appears more than once"),
            Self::DuplicateDependency { module, dependency } => write!(
                f,
                "module `{module}` lists dependency `{dependency}` more than once"
            ),
            Self::MissingOrForwardDependency { module, dependency } => write!(
                f,
                "module `{module}` depends on `{dependency}`, which has not appeared earlier"
            ),
        }
    }
}

impl std::error::Error for ContractError {}

/// Validate semantics that are independent of the storage/binding format.
///
/// In particular, dependencies must refer to modules that were already
/// constructed. This preserves the ordering semantics of the Scala constructor.
pub fn validate_contract<S: CpgSchemaContract>(schema: &S) -> Result<(), ContractError> {
    if schema.domain_short_name().is_empty() {
        return Err(ContractError::EmptyDomainShortName);
    }
    if schema.base_package().is_empty() {
        return Err(ContractError::EmptyBasePackage);
    }

    for (index, package) in schema.additional_traversals_packages().iter().enumerate() {
        if package.is_empty() {
            return Err(ContractError::EmptyTraversalPackage { index });
        }
    }

    let mut initialized = BTreeSet::<&str>::new();

    for (index, module) in schema.modules().iter().enumerate() {
        let name = module.name();
        if name.is_empty() {
            return Err(ContractError::EmptyModuleName { index });
        }
        if !initialized.insert(name) {
            return Err(ContractError::DuplicateModule {
                name: name.to_owned(),
            });
        }

        let mut dependencies = BTreeSet::<&str>::new();
        for dependency in module.dependencies() {
            if !dependencies.insert(*dependency) {
                return Err(ContractError::DuplicateDependency {
                    module: name.to_owned(),
                    dependency: (*dependency).to_owned(),
                });
            }
            if !initialized.contains(dependency) {
                return Err(ContractError::MissingOrForwardDependency {
                    module: name.to_owned(),
                    dependency: (*dependency).to_owned(),
                });
            }
        }
    }

    Ok(())
}
