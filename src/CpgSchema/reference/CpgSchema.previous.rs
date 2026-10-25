use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeSet, fmt, sync::LazyLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(String);

impl Error {
    fn invalid(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Configuration passed to the schema builder by `CpgSchema.instance`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, /*Deserialize*/ )]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CpgSchemaConfig {
    pub domain_short_name: String,
    pub base_package: String,
    pub additional_traversals_packages: Vec<String>,
}

impl Default for CpgSchemaConfig {
    fn default() -> Self {
        Self {
            domain_short_name: "Cpg".to_owned(),
            base_package: "io.shiftleft.codepropertygraph.generated".to_owned(),
            additional_traversals_packages: vec!["io.shiftleft".to_owned()],
        }
    }
}

/// Validates the exact configuration used by the Scala companion object.
#[jsonschema::validator(
    schema = r#"{
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "properties": {
            "domainShortName": {
                "const": "Cpg"
            },
            "basePackage": {
                "const": "io.shiftleft.codepropertygraph.generated"
            },
            "additionalTraversalsPackages": {
                "const": ["io.shiftleft"]
            }
        },
        "required": [
            "domainShortName",
            "basePackage",
            "additionalTraversalsPackages"
        ],
        "additionalProperties": false
    }"#,
    draft = Draft202012
)]
pub struct CpgSchemaConfigValidator;

pub fn validate_config(value: &Value) -> Result<()> {
    CpgSchemaConfigValidator::validate(value)
        .map_err(|error| Error::invalid(format!("invalid CpgSchema configuration: {error}")))
}

/// A schema module created by one of the Scala expressions such as
/// `Base(builder)` or `Method(builder, base, typeSchema, fs)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, /*Deserialize*/ )]
pub struct SchemaModule {
    pub name: &'static str,
    pub dependencies: Vec<&'static str>,
}

/// Standalone representation of FlatGraph's final `Schema` value.
///
/// The module internals belong to their individual source files; this type
/// captures exactly what `CpgSchema.scala` itself contributes: builder identity,
/// module construction order, and dependency wiring.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, /*Deserialize*/ )]
pub struct Schema {
    pub domain_short_name: String,
    pub base_package: String,
    pub additional_traversals_packages: Vec<String>,
    pub modules: Vec<SchemaModule>,
}

#[derive(Clone, Debug)]
pub struct SchemaBuilder {
    config: CpgSchemaConfig,
    modules: Vec<SchemaModule>,
    names: BTreeSet<&'static str>,
}

impl SchemaBuilder {
    pub fn new(
        domain_short_name: impl Into<String>,
        base_package: impl Into<String>,
        additional_traversals_packages: Vec<String>,
    ) -> Self {
        Self {
            config: CpgSchemaConfig {
                domain_short_name: domain_short_name.into(),
                base_package: base_package.into(),
                additional_traversals_packages,
            },
            modules: Vec::new(),
            names: BTreeSet::new(),
        }
    }

    /// Register one schema module.
    ///
    /// Dependencies must already have been constructed, which enforces the same
    /// initialization ordering implied by the Scala constructor.
    fn register_module(
        &mut self,
        name: &'static str,
        dependencies: &[&SchemaModule],
    ) -> Result<SchemaModule> {
        if !self.names.insert(name) {
            return Err(Error::invalid(format!(
                "schema module `{name}` was initialized more than once"
            )));
        }

        for dependency in dependencies {
            if !self.names.contains(dependency.name) {
                return Err(Error::invalid(format!(
                    "schema module `{name}` depends on `{}`, \
                     but that module has not been initialized",
                    dependency.name
                )));
            }
        }

        let module = SchemaModule {
            name,
            dependencies: dependencies
                .iter()
                .map(|dependency| dependency.name)
                .collect(),
        };

        self.modules.push(module.clone());
        Ok(module)
    }

    pub fn build(self) -> Result<Schema> {
        let config_value = serde_json::to_value(&self.config)
            .map_err(|error| Error::invalid(error.to_string()))?;
        validate_config(&config_value)?;

        Ok(Schema {
            domain_short_name: self.config.domain_short_name,
            base_package: self.config.base_package,
            additional_traversals_packages: self.config.additional_traversals_packages,
            modules: self.modules,
        })
    }
}

/// Rust equivalent of Scala's `class CpgSchema(builder: SchemaBuilder)`.
///
/// Field names follow the Scala source as closely as Rust naming conventions
/// permit. Each value records the corresponding schema module and its direct
/// dependencies.
#[derive(Clone, Debug)]
pub struct CpgSchema {
    pub base: SchemaModule,
    pub fs: SchemaModule,
    pub namespaces: SchemaModule,

    pub operators: SchemaModule,
    pub configuration: SchemaModule,
    pub meta_data: SchemaModule,

    pub type_schema: SchemaModule,
    pub method: SchemaModule,
    pub ast: SchemaModule,
    pub call_graph: SchemaModule,

    pub cfg: SchemaModule,
    pub dominators: SchemaModule,
    pub pdg: SchemaModule,

    pub shortcuts: SchemaModule,

    pub source_specific: SchemaModule,
    pub tags: SchemaModule,
    pub binding: SchemaModule,
    pub annotation: SchemaModule,
    pub finding: SchemaModule,
    pub hidden: SchemaModule,
    pub proto_serialize: SchemaModule,
}

impl CpgSchema {
    pub fn new(builder: &mut SchemaBuilder) -> Result<Self> {
        // val base = Base(builder)
        let base = builder.register_module("Base", &[])?;

        // val fs = FileSystem(builder, base)
        let fs = builder.register_module("FileSystem", &[&base])?;

        // val namespaces = Namespace(builder, base, fs)
        let namespaces = builder.register_module("Namespace", &[&base, &fs])?;

        // val operators = Operators(builder)
        let operators = builder.register_module("Operators", &[])?;

        // val configuration = Configuration(builder, base)
        let configuration = builder.register_module("Configuration", &[&base])?;

        // val metaData = MetaData(builder, base)
        let meta_data = builder.register_module("MetaData", &[&base])?;

        // val typeSchema = Type(builder, base, fs)
        let type_schema = builder.register_module("Type", &[&base, &fs])?;

        // val method = Method(builder, base, typeSchema, fs)
        let method =
            builder.register_module("Method", &[&base, &type_schema, &fs])?;

        // val ast = Ast(builder, base, namespaces, method, typeSchema, fs)
        let ast = builder.register_module(
            "Ast",
            &[&base, &namespaces, &method, &type_schema, &fs],
        )?;

        // val callGraph = CallGraph(builder, method, ast)
        let call_graph = builder.register_module("CallGraph", &[&method, &ast])?;

        // val cfg = Cfg(builder, method, ast)
        let cfg = builder.register_module("Cfg", &[&method, &ast])?;

        // val dominators = Dominators(builder, method, ast)
        let dominators =
            builder.register_module("Dominators", &[&method, &ast])?;

        // val pdg = Pdg(builder, method, ast)
        let pdg = builder.register_module("Pdg", &[&method, &ast])?;

        // val shortcuts = Shortcuts(builder, base, method, ast, typeSchema, fs)
        let shortcuts = builder.register_module(
            "Shortcuts",
            &[&base, &method, &ast, &type_schema, &fs],
        )?;

        // val sourceSpecific = Comment(builder, base, ast, fs)
        let source_specific =
            builder.register_module("Comment", &[&base, &ast, &fs])?;

        // val tags = Tags(builder, base, typeSchema, method, ast, fs, callGraph)
        let tags = builder.register_module(
            "Tags",
            &[&base, &type_schema, &method, &ast, &fs, &call_graph],
        )?;

        // val binding = Binding(builder, base, typeSchema, method, callGraph)
        let binding = builder.register_module(
            "Binding",
            &[&base, &type_schema, &method, &call_graph],
        )?;

        // val annotation =
        //   Annotation(builder, base, method, typeSchema, ast, shortcuts)
        let annotation = builder.register_module(
            "Annotation",
            &[&base, &method, &type_schema, &ast, &shortcuts],
        )?;

        // val finding = Finding(builder, base)
        let finding = builder.register_module("Finding", &[&base])?;

        // val hidden = Hidden(
        //   builder, base, method, typeSchema, ast, cfg, fs,
        //   callGraph, pdg, tags, shortcuts
        // )
        let hidden = builder.register_module(
            "Hidden",
            &[
                &base,
                &method,
                &type_schema,
                &ast,
                &cfg,
                &fs,
                &call_graph,
                &pdg,
                &tags,
                &shortcuts,
            ],
        )?;

        // val protoSerialize = ProtoSerialize(builder, ast)
        let proto_serialize =
            builder.register_module("ProtoSerialize", &[&ast])?;

        Ok(Self {
            base,
            fs,
            namespaces,
            operators,
            configuration,
            meta_data,
            type_schema,
            method,
            ast,
            call_graph,
            cfg,
            dominators,
            pdg,
            shortcuts,
            source_specific,
            tags,
            binding,
            annotation,
            finding,
            hidden,
            proto_serialize,
        })
    }
}

/// Rust equivalent of Scala's `CpgSchema.instance`.
pub fn build_instance() -> Result<Schema> {
    let config = CpgSchemaConfig::default();

    let mut builder = SchemaBuilder::new(
        config.domain_short_name,
        config.base_package,
        config.additional_traversals_packages,
    );

    CpgSchema::new(&mut builder)?;
    builder.build()
}

/// Lazy singleton equivalent of the Scala companion object's `val instance`.
pub static INSTANCE: LazyLock<Schema> = LazyLock::new(|| {
    build_instance().expect("built-in CPG schema configuration must be valid")
});

pub fn instance() -> &'static Schema {
    &INSTANCE
}

/// Rust equivalent of Scala's `CpgSchema.PropertyDefaults`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_configuration_is_valid() {
        let value = serde_json::to_value(CpgSchemaConfig::default()).unwrap();
        validate_config(&value).unwrap();
    }

    #[test]
    fn instance_preserves_scala_builder_configuration() {
        let schema = build_instance().unwrap();

        assert_eq!(schema.domain_short_name, "Cpg");
        assert_eq!(
            schema.base_package,
            "io.shiftleft.codepropertygraph.generated"
        );
        assert_eq!(
            schema.additional_traversals_packages,
            vec!["io.shiftleft".to_owned()]
        );
    }

    #[test]
    fn modules_are_constructed_in_scala_order() {
        let schema = build_instance().unwrap();
        let names: Vec<_> = schema.modules.iter().map(|module| module.name).collect();

        assert_eq!(
            names,
            vec![
                "Base",
                "FileSystem",
                "Namespace",
                "Operators",
                "Configuration",
                "MetaData",
                "Type",
                "Method",
                "Ast",
                "CallGraph",
                "Cfg",
                "Dominators",
                "Pdg",
                "Shortcuts",
                "Comment",
                "Tags",
                "Binding",
                "Annotation",
                "Finding",
                "Hidden",
                "ProtoSerialize",
            ]
        );
    }

    #[test]
    fn hidden_preserves_all_source_dependencies() {
        let schema = build_instance().unwrap();
        let hidden = schema
            .modules
            .iter()
            .find(|module| module.name == "Hidden")
            .unwrap();

        assert_eq!(
            hidden.dependencies,
            vec![
                "Base",
                "Method",
                "Type",
                "Ast",
                "Cfg",
                "FileSystem",
                "CallGraph",
                "Pdg",
                "Tags",
                "Shortcuts",
            ]
        );
    }

    #[test]
    fn property_defaults_match_scala() {
        assert_eq!(property_defaults::STRING, "<empty>");
        assert!(!property_defaults::BOOLEAN);
        assert_eq!(property_defaults::BYTE, -1);
        assert_eq!(property_defaults::SHORT, -1);
        assert_eq!(property_defaults::INT, -1);
        assert_eq!(property_defaults::LONG, -1);
        assert_eq!(property_defaults::FLOAT, 0.0);
        assert_eq!(property_defaults::DOUBLE, 0.0);
        assert_eq!(property_defaults::CHAR, '?');
    }
}
