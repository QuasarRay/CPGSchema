//! Compile-time provider choices for JSON construction, interpretation and export.
use crate::{
    graph::{Diagnostic, Graph, ValidationOptions},
    model::{Result, Schema, SchemaBuilder},
    value::{Scalar, ValueType},
};
use cgp::prelude::*;
#[cgp_component(SchemaComposer)]
pub trait CanComposeSchema {
    fn compose_schema(&self, builder: &mut SchemaBuilder) -> Result<crate::CpgSchema::CpgSchema>;
}
#[cgp_component(SchemaProvider)]
pub trait CanProvideSchema {
    fn provide_schema(&self) -> Result<Schema>;
}
#[cgp_component(SchemaValidator)]
pub trait CanValidateSchema {
    fn validate_schema(&self, schema: &Schema) -> Result<()>;
}
#[cgp_component(GraphValidator)]
pub trait CanValidateGraph {
    fn validate_graph(
        &self,
        schema: &Schema,
        graph: &Graph,
        options: &ValidationOptions,
    ) -> Vec<Diagnostic>;
}
#[cgp_component(ValueInterpreter)]
pub trait CanInterpretValue {
    fn value_type(&self, value: &Scalar) -> ValueType;
}
#[cgp_component(ProtoExporter)]
pub trait CanExportProto {
    fn export_proto(&self, schema: &Schema) -> Result<String>;
}
#[cgp_new_provider]
impl<Context> SchemaComposer<Context> for ComposeJsonModules {
    fn compose_schema(
        _context: &Context,
        builder: &mut SchemaBuilder,
    ) -> Result<crate::CpgSchema::CpgSchema> {
        crate::CpgSchema::CpgSchema::new(builder)
    }
}
#[cgp_new_provider]
impl<Context: CanComposeSchema> SchemaProvider<Context> for BuildJsonSchema {
    fn provide_schema(context: &Context) -> Result<Schema> {
        let document: serde_json::Value = serde_json::from_str(crate::CpgSchema::DOCUMENT)
            .map_err(|e| crate::Error::new("CpgSchema", e.to_string()))?;
        let mut builder = crate::interpreter::configured_builder(&document)?;
        context.compose_schema(&mut builder)?;
        builder.build()
    }
}
#[cgp_new_provider]
impl<Context> SchemaValidator<Context> for ValidateJsonSchema {
    fn validate_schema(_context: &Context, schema: &Schema) -> Result<()> {
        schema.validate()
    }
}
#[cgp_new_provider]
impl<Context> GraphValidator<Context> for ValidateJsonGraph {
    fn validate_graph(
        _context: &Context,
        schema: &Schema,
        graph: &Graph,
        options: &ValidationOptions,
    ) -> Vec<Diagnostic> {
        crate::graph::validate_graph(schema, graph, options)
    }
}
#[cgp_new_provider]
impl<Context> ValueInterpreter<Context> for InterpretJsonValue {
    fn value_type(_context: &Context, value: &Scalar) -> ValueType {
        value.value_type()
    }
}
#[cgp_new_provider]
impl<Context> ProtoExporter<Context> for ExportJsonProto {
    fn export_proto(_context: &Context, schema: &Schema) -> Result<String> {
        crate::Protogen::generate(schema)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct JsonContext;
delegate_components! {JsonContext {
    SchemaComposerComponent:ComposeJsonModules,
    SchemaProviderComponent:BuildJsonSchema,
    SchemaValidatorComponent:ValidateJsonSchema,
    GraphValidatorComponent:ValidateJsonGraph,
    ValueInterpreterComponent:InterpretJsonValue,
    ProtoExporterComponent:ExportJsonProto,
}}
