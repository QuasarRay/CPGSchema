//! JSON Schema Draft 2020-12 dialect, local resources and real CPG assertions.
use crate::{
    graph::{ConstructionStage, ValidationOptions},
    model::{Error, Result, Schema},
};
use jsonschema::{Draft, Registry, Validator};
use serde_json::{Map, Value, json};
use std::sync::OnceLock;
pub const DIALECT_URI: &str = "https://quasarray.github.io/CPGSchema/dialect/1";
pub const VOCABULARY_URI: &str = "https://quasarray.github.io/CPGSchema/vocabulary/1";
pub const MODULE_META_URI: &str = "https://quasarray.github.io/CPGSchema/meta/module/1";
pub const BUNDLE_META_URI: &str = "https://quasarray.github.io/CPGSchema/meta/bundle/1";
pub const GRAPH_META_URI: &str = "https://quasarray.github.io/CPGSchema/meta/graph/1";
pub const VOCABULARY_META_URI: &str = "https://quasarray.github.io/CPGSchema/meta/vocabulary/1";
pub const DIALECT_DOCUMENT: &str = include_str!("../schemas/dialect.meta.schema.json");
pub const VOCABULARY_DOCUMENT: &str = include_str!("../schemas/vocabulary.meta.schema.json");
pub const MODULE_META_DOCUMENT: &str = include_str!("../schemas/module.meta.schema.json");
pub const BUNDLE_META_DOCUMENT: &str = include_str!("../schemas/bundle.meta.schema.json");
pub const GRAPH_META_DOCUMENT: &str = include_str!("../schemas/graph.meta.schema.json");
fn parse(s: &str) -> Result<Value> {
    serde_json::from_str(s).map_err(|e| Error::new("dialect resource", e.to_string()))
}
fn registry() -> Result<Registry<'static>> {
    let mut r = Registry::new();
    for (uri, source) in [
        (DIALECT_URI, DIALECT_DOCUMENT),
        (VOCABULARY_META_URI, VOCABULARY_DOCUMENT),
        (MODULE_META_URI, MODULE_META_DOCUMENT),
        (BUNDLE_META_URI, BUNDLE_META_DOCUMENT),
        (GRAPH_META_URI, GRAPH_META_DOCUMENT),
    ] {
        r = r
            .add(uri, parse(source)?)
            .map_err(|e| Error::new(uri, e.to_string()))?;
    }
    r.prepare()
        .map_err(|e| Error::new("registry", e.to_string()))
}
fn syntax_validator(source: &str) -> Result<Validator> {
    let r = registry()?;
    jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(&r)
        .with_vocabulary(VOCABULARY_URI)
        .build(&parse(source)?)
        .map_err(|e| Error::new("meta-schema compilation", e.to_string()))
}
fn validate_with(v: &std::result::Result<Validator, Error>, document: &Value) -> Result<()> {
    v.as_ref()
        .map_err(Clone::clone)?
        .validate(document)
        .map_err(|e| Error::new(e.instance_path().to_string(), e.to_string()))
}
pub fn validate_module_syntax(document: &Value) -> Result<()> {
    static V: OnceLock<Result<Validator>> = OnceLock::new();
    validate_with(
        V.get_or_init(|| syntax_validator(MODULE_META_DOCUMENT)),
        document,
    )
}
pub fn validate_bundle_syntax(document: &Value) -> Result<()> {
    static V: OnceLock<Result<Validator>> = OnceLock::new();
    validate_with(
        V.get_or_init(|| syntax_validator(BUNDLE_META_DOCUMENT)),
        document,
    )
}
pub fn validate_graph_syntax(document: &Value) -> Result<()> {
    static V: OnceLock<Result<Validator>> = OnceLock::new();
    validate_with(
        V.get_or_init(|| syntax_validator(GRAPH_META_DOCUMENT)),
        document,
    )
}
/// Validate a schema document against its declared custom dialect, distinct from interpreting it.
pub fn validate_document_metaschema(document: &Value) -> Result<()> {
    let r = registry()?;
    jsonschema::meta::options()
        .with_registry(&r)
        .validate(document)
        .map_err(|e| Error::new("$schema", e.to_string()))
}
struct SchemaAssertion {
    enabled: bool,
}
impl<'i> jsonschema::Keyword<'i> for SchemaAssertion {
    fn validate(
        &self,
        instance: &'i Value,
    ) -> std::result::Result<(), jsonschema::ValidationError<'i>> {
        if !self.enabled {
            return Ok(());
        }
        crate::interpreter::interpret_bundle(instance)
            .map(|_| ())
            .map_err(|e| jsonschema::ValidationError::custom(e.to_string()))
    }
    fn is_valid(&self, instance: &'i Value) -> bool {
        !self.enabled || crate::interpreter::interpret_bundle(instance).is_ok()
    }
}
fn schema_keyword<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    _path: jsonschema::paths::Location,
) -> std::result::Result<Box<dyn for<'i> jsonschema::Keyword<'i>>, jsonschema::ValidationError<'a>>
{
    let enabled = value
        .as_bool()
        .ok_or_else(|| jsonschema::ValidationError::schema("cpg:schema must be boolean"))?;
    Ok(Box::new(SchemaAssertion { enabled }))
}
struct GraphAssertion {
    schema: Schema,
    options: ValidationOptions,
}
impl<'i> jsonschema::Keyword<'i> for GraphAssertion {
    fn validate(
        &self,
        instance: &'i Value,
    ) -> std::result::Result<(), jsonschema::ValidationError<'i>> {
        let result = (|| -> Result<()> {
            validate_graph_syntax(instance)?;
            let graph = crate::codec::decode_graph(instance)?;
            let ds = crate::graph::validate_graph(&self.schema, &graph, &self.options);
            if ds.is_empty() {
                Ok(())
            } else {
                Err(Error::new(
                    &ds[0].path,
                    ds.iter()
                        .map(|d| format!("{}: {}", d.path, d.message))
                        .collect::<Vec<_>>()
                        .join("; "),
                ))
            }
        })();
        result.map_err(|e| jsonschema::ValidationError::custom(e.to_string()))
    }
    fn is_valid(&self, instance: &'i Value) -> bool {
        self.validate(instance).is_ok()
    }
}
fn graph_keyword<'a>(
    _parent: &'a Map<String, Value>,
    value: &'a Value,
    _path: jsonschema::paths::Location,
) -> std::result::Result<Box<dyn for<'i> jsonschema::Keyword<'i>>, jsonschema::ValidationError<'a>>
{
    let compile = || -> Result<GraphAssertion> {
        let o = value
            .as_object()
            .ok_or_else(|| Error::new("cpg:graph", "expected options object"))?;
        let schema = crate::interpreter::interpret_bundle(
            o.get("schema")
                .ok_or_else(|| Error::new("cpg:graph", "missing schema bundle"))?,
        )?;
        let stage = match o.get("stage").and_then(Value::as_str) {
            Some("Frontend") => ConstructionStage::Frontend,
            Some("Materialized") => ConstructionStage::Materialized,
            _ => {
                return Err(Error::new(
                    "cpg:graph",
                    "stage must be explicit Frontend or Materialized",
                ));
            }
        };
        let layers = o
            .get("layers")
            .and_then(Value::as_array)
            .ok_or_else(|| Error::new("cpg:graph", "layers must be explicit array"))?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| Error::new("cpg:graph", "layer must be string"))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(GraphAssertion {
            schema,
            options: ValidationOptions::new(stage, layers),
        })
    };
    compile()
        .map(|v| Box::new(v) as Box<dyn for<'i> jsonschema::Keyword<'i>>)
        .map_err(|e| jsonschema::ValidationError::schema(e.to_string()))
}
/// A standalone dialect compilation context, with no native-package dependency or network retriever.
#[derive(Clone, Copy, Debug, Default)]
pub struct Dialect;
impl Dialect {
    pub fn compile(&self, schema: &Value) -> Result<Validator> {
        let r = registry()?;
        jsonschema::options()
            .with_registry(&r)
            .with_vocabulary(VOCABULARY_URI)
            .with_keyword("cpg:schema", schema_keyword)
            .with_keyword("cpg:graph", graph_keyword)
            .build(schema)
            .map_err(|e| Error::new("schema compilation", e.to_string()))
    }
    pub fn compile_graph(
        &self,
        bundle: &Value,
        stage: ConstructionStage,
        layers: &[String],
    ) -> Result<Validator> {
        self.compile(&json!({"$schema":DIALECT_URI,"cpg:graph":{"schema":bundle,"stage":match stage{ConstructionStage::Frontend=>"Frontend",ConstructionStage::Materialized=>"Materialized"},"layers":layers}}))
    }
    pub fn compile_schema_validator(&self) -> Result<Validator> {
        self.compile(&json!({"$schema":DIALECT_URI,"cpg:schema":true}))
    }
    pub fn validate_schema(&self, bundle: &Value) -> Result<Schema> {
        crate::interpreter::interpret_bundle(bundle)
    }
}
