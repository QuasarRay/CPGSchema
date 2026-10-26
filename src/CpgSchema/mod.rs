//! Native singleton construction and original CpgSchema companion responsibilities.
use crate::model::{Result, SchemaBuilder};
#[derive(Clone, Debug)]
pub struct CpgSchema {
    pub base: crate::Base::Schema,
    pub fs: crate::FileSystem::Schema,
    pub namespaces: crate::Namespace::Schema,
    pub operators: crate::Operators::Schema,
    pub configuration: crate::Configuration::Schema,
    pub metaData: crate::MetaData::Schema,
    pub typeSchema: crate::Type::Schema,
    pub method: crate::Method::Schema,
    pub ast: crate::Ast::Schema,
    pub callGraph: crate::CallGraph::Schema,
    pub cfg: crate::Cfg::Schema,
    pub dominators: crate::Dominators::Schema,
    pub pdg: crate::Pdg::Schema,
    pub shortcuts: crate::Shortcuts::Schema,
    pub sourceSpecific: crate::Comment::Schema,
    pub tags: crate::Tags::Schema,
    pub binding: crate::Binding::Schema,
    pub annotation: crate::Annotation::Schema,
    pub finding: crate::Finding::Schema,
    pub hidden: crate::Hidden::Schema,
    pub protoSerialize: crate::ProtoSerialize::Schema,
}
impl CpgSchema {
    pub fn new(builder: &mut SchemaBuilder) -> Result<Self> {
        let base = crate::Base::Schema::new(builder)?;
        let fs = crate::FileSystem::Schema::new(builder, &base)?;
        let namespaces = crate::Namespace::Schema::new(builder, &base, &fs)?;
        let operators = crate::Operators::Schema::new(builder)?;
        let configuration = crate::Configuration::Schema::new(builder, &base)?;
        let metaData = crate::MetaData::Schema::new(builder, &base)?;
        let typeSchema = crate::Type::Schema::new(builder, &base, &fs)?;
        let method = crate::Method::Schema::new(builder, &base, &typeSchema, &fs)?;
        let ast = crate::Ast::Schema::new(builder, &base, &namespaces, &method, &typeSchema, &fs)?;
        let callGraph = crate::CallGraph::Schema::new(builder, &method, &ast)?;
        let cfg = crate::Cfg::Schema::new(builder, &method, &ast)?;
        let dominators = crate::Dominators::Schema::new(builder, &method, &ast)?;
        let pdg = crate::Pdg::Schema::new(builder, &method, &ast)?;
        let shortcuts =
            crate::Shortcuts::Schema::new(builder, &base, &method, &ast, &typeSchema, &fs)?;
        let sourceSpecific = crate::Comment::Schema::new(builder, &base, &ast, &fs)?;
        let tags =
            crate::Tags::Schema::new(builder, &base, &typeSchema, &method, &ast, &fs, &callGraph)?;
        let binding =
            crate::Binding::Schema::new(builder, &base, &typeSchema, &method, &callGraph)?;
        let annotation =
            crate::Annotation::Schema::new(builder, &base, &method, &typeSchema, &ast, &shortcuts)?;
        let finding = crate::Finding::Schema::new(builder, &base)?;
        let hidden = crate::Hidden::Schema::new(
            builder,
            &base,
            &method,
            &typeSchema,
            &ast,
            &cfg,
            &fs,
            &callGraph,
            &pdg,
            &tags,
            &shortcuts,
        )?;
        let protoSerialize = crate::ProtoSerialize::Schema::new(builder, &ast)?;
        Ok(Self {
            base,
            fs,
            namespaces,
            operators,
            configuration,
            metaData,
            typeSchema,
            method,
            ast,
            callGraph,
            cfg,
            dominators,
            pdg,
            shortcuts,
            sourceSpecific,
            tags,
            binding,
            annotation,
            finding,
            hidden,
            protoSerialize,
        })
    }
}
pub fn instance() -> Result<crate::model::Schema> {
    let mut builder = SchemaBuilder::new(
        "Cpg",
        "io.shiftleft.codepropertygraph.generated",
        &["io.shiftleft"],
    );
    CpgSchema::new(&mut builder)?;
    builder.build()
}
pub mod PropertyDefaults {
    #![allow(non_upper_case_globals, reason = "Scala companion constant spelling")]
    pub const String: &str = "<empty>";
    pub const Boolean: bool = false;
    pub const Byte: i8 = -1;
    pub const Short: i16 = -1;
    pub const Int: i32 = -1;
    pub const Long: i64 = -1;
    pub const Float: f32 = 0.0;
    pub const Double: f64 = 0.0;
    pub const Char: u16 = 63;
}
