//! Native Rust implementation of the CpgSchema contract.
//!
//! No JSON or serde dependency is required.

use crate::contracts::{
    CpgSchemaContract, PropertyDefaultsContract, SchemaModuleContract,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeModule {
    pub name: &'static str,
    pub dependencies: &'static [&'static str],
}

impl SchemaModuleContract for NativeModule {
    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    #[inline]
    fn dependencies(&self) -> &[&str] {
        self.dependencies
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativePropertyDefaults {
    pub string: &'static str,
    pub boolean: bool,
    pub byte: i8,
    pub short: i16,
    pub int: i32,
    pub long: i64,
    pub float: f32,
    pub double: f64,
    pub char_: char,
}

impl PropertyDefaultsContract for NativePropertyDefaults {
    #[inline] fn string_default(&self) -> &str { self.string }
    #[inline] fn boolean_default(&self) -> bool { self.boolean }
    #[inline] fn byte_default(&self) -> i8 { self.byte }
    #[inline] fn short_default(&self) -> i16 { self.short }
    #[inline] fn int_default(&self) -> i32 { self.int }
    #[inline] fn long_default(&self) -> i64 { self.long }
    #[inline] fn float_default(&self) -> f32 { self.float }
    #[inline] fn double_default(&self) -> f64 { self.double }
    #[inline] fn char_default(&self) -> char { self.char_ }
}

pub const PROPERTY_DEFAULTS: NativePropertyDefaults = NativePropertyDefaults {
    string: "<empty>",
    boolean: false,
    byte: -1,
    short: -1,
    int: -1,
    long: -1,
    float: 0.0,
    double: 0.0,
    char_: '?',
};

/// Compatibility constants matching Scala's `CpgSchema.PropertyDefaults`.
pub mod property_defaults {
    pub const STRING: &str = "<empty>";
    pub const BOOLEAN: bool = false;
    pub const BYTE: i8 = -1;
    pub const SHORT: i16 = -1;
    pub const INT: i32 = -1;
    pub const LONG: i64 = -1;
    pub const FLOAT: f32 = 0.0;
    pub const DOUBLE: f64 = 0.0;
    pub const CHAR: char = '?';
}

pub const MODULES: &[NativeModule] = &[
    NativeModule { name: "Base", dependencies: &[] },
    NativeModule { name: "FileSystem", dependencies: &["Base"] },
    NativeModule { name: "Namespace", dependencies: &["Base", "FileSystem"] },
    NativeModule { name: "Operators", dependencies: &[] },
    NativeModule { name: "Configuration", dependencies: &["Base"] },
    NativeModule { name: "MetaData", dependencies: &["Base"] },
    NativeModule { name: "Type", dependencies: &["Base", "FileSystem"] },
    NativeModule { name: "Method", dependencies: &["Base", "Type", "FileSystem"] },
    NativeModule { name: "Ast", dependencies: &["Base", "Namespace", "Method", "Type", "FileSystem"] },
    NativeModule { name: "CallGraph", dependencies: &["Method", "Ast"] },
    NativeModule { name: "Cfg", dependencies: &["Method", "Ast"] },
    NativeModule { name: "Dominators", dependencies: &["Method", "Ast"] },
    NativeModule { name: "Pdg", dependencies: &["Method", "Ast"] },
    NativeModule { name: "Shortcuts", dependencies: &["Base", "Method", "Ast", "Type", "FileSystem"] },
    NativeModule { name: "Comment", dependencies: &["Base", "Ast", "FileSystem"] },
    NativeModule { name: "Tags", dependencies: &["Base", "Type", "Method", "Ast", "FileSystem", "CallGraph"] },
    NativeModule { name: "Binding", dependencies: &["Base", "Type", "Method", "CallGraph"] },
    NativeModule { name: "Annotation", dependencies: &["Base", "Method", "Type", "Ast", "Shortcuts"] },
    NativeModule { name: "Finding", dependencies: &["Base"] },
    NativeModule {
        name: "Hidden",
        dependencies: &[
            "Base", "Method", "Type", "Ast", "Cfg", "FileSystem",
            "CallGraph", "Pdg", "Tags", "Shortcuts",
        ],
    },
    NativeModule { name: "ProtoSerialize", dependencies: &["Ast"] },
];

#[derive(Clone, Copy, Debug, Default)]
pub struct NativeCpgSchema;

pub const INSTANCE: NativeCpgSchema = NativeCpgSchema;

impl CpgSchemaContract for NativeCpgSchema {
    type Module = NativeModule;
    type Defaults = NativePropertyDefaults;

    #[inline]
    fn domain_short_name(&self) -> &str {
        "Cpg"
    }

    #[inline]
    fn base_package(&self) -> &str {
        "io.shiftleft.codepropertygraph.generated"
    }

    #[inline]
    fn additional_traversals_packages(&self) -> &[&str] {
        &["io.shiftleft"]
    }

    #[inline]
    fn modules(&self) -> &[Self::Module] {
        MODULES
    }

    #[inline]
    fn property_defaults(&self) -> &Self::Defaults {
        &PROPERTY_DEFAULTS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{validate_contract, CpgSchemaContract, PropertyDefaultsContract};

    #[test]
    fn native_contract_is_valid() {
        validate_contract(&INSTANCE).unwrap();
    }

    #[test]
    fn canonical_builder_configuration_matches_scala() {
        assert_eq!(INSTANCE.domain_short_name(), "Cpg");
        assert_eq!(
            INSTANCE.base_package(),
            "io.shiftleft.codepropertygraph.generated"
        );
        assert_eq!(
            INSTANCE.additional_traversals_packages(),
            &["io.shiftleft"]
        );
    }

    #[test]
    fn module_order_matches_scala() {
        let names: Vec<_> = INSTANCE.modules().iter().map(|m| m.name).collect();
        assert_eq!(
            names,
            vec![
                "Base", "FileSystem", "Namespace", "Operators", "Configuration",
                "MetaData", "Type", "Method", "Ast", "CallGraph", "Cfg",
                "Dominators", "Pdg", "Shortcuts", "Comment", "Tags", "Binding",
                "Annotation", "Finding", "Hidden", "ProtoSerialize",
            ]
        );
    }

    #[test]
    fn property_defaults_match_scala() {
        let d = INSTANCE.property_defaults();
        assert_eq!(d.string_default(), "<empty>");
        assert!(!d.boolean_default());
        assert_eq!(d.byte_default(), -1);
        assert_eq!(d.short_default(), -1);
        assert_eq!(d.int_default(), -1);
        assert_eq!(d.long_default(), -1);
        assert_eq!(d.float_default(), 0.0);
        assert_eq!(d.double_default(), 0.0);
        assert_eq!(d.char_default(), '?');
    }
}
