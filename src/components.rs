//! Compile-time provider choices for native construction, interpretation and export.
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
impl<Context> SchemaComposer<Context> for ComposeNativeModules {
    fn compose_schema(
        _context: &Context,
        builder: &mut SchemaBuilder,
    ) -> Result<crate::CpgSchema::CpgSchema> {
        crate::CpgSchema::CpgSchema::new(builder)
    }
}
#[cgp_new_provider]
impl<Context: CanComposeSchema> SchemaProvider<Context> for BuildNativeSchema {
    fn provide_schema(context: &Context) -> Result<Schema> {
        let mut builder = SchemaBuilder::new(
            "Cpg",
            "io.shiftleft.codepropertygraph.generated",
            &["io.shiftleft"],
        );
        context.compose_schema(&mut builder)?;
        builder.build()
    }
}
#[cgp_new_provider]
impl<Context> SchemaValidator<Context> for ValidateNativeSchema {
    fn validate_schema(_context: &Context, schema: &Schema) -> Result<()> {
        schema.validate()
    }
}
#[cgp_new_provider]
impl<Context> GraphValidator<Context> for ValidateNativeGraph {
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
impl<Context> ValueInterpreter<Context> for InterpretNativeValue {
    fn value_type(_context: &Context, value: &Scalar) -> ValueType {
        value.value_type()
    }
}
#[cgp_new_provider]
impl<Context> ProtoExporter<Context> for ExportNativeProto {
    fn export_proto(_context: &Context, schema: &Schema) -> Result<String> {
        crate::Protogen::generate(schema)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeContext;
delegate_components! {NativeContext {
    SchemaComposerComponent:ComposeNativeModules,
    SchemaProviderComponent:BuildNativeSchema,
    SchemaValidatorComponent:ValidateNativeSchema,
    GraphValidatorComponent:ValidateNativeGraph,
    ValueInterpreterComponent:InterpretNativeValue,
    ProtoExporterComponent:ExportNativeProto,
}}
