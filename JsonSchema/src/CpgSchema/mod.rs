//! JSON-owned composition and canonical builder configuration.
use crate::model::{Result, SchemaBuilder};
pub const DOCUMENT: &str = include_str!("schema.json");
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
        let document: serde_json::Value = serde_json::from_str(DOCUMENT)
            .map_err(|e| crate::Error::new("CpgSchema", e.to_string()))?;
        for module in document["cpg:composition"]
            .as_array()
            .ok_or_else(|| crate::Error::new("CpgSchema", "missing composition"))?
        {
            let name = module
                .as_str()
                .ok_or_else(|| crate::Error::new("CpgSchema", "module name must be string"))?;
            crate::interpreter::apply_module(
                builder,
                &crate::interpreter::canonical_module(name)?,
            )?;
        }
        Ok(Self {
            base: crate::Base::Schema::from_builder(builder)?,
            fs: crate::FileSystem::Schema::from_builder(builder)?,
            namespaces: crate::Namespace::Schema::from_builder(builder)?,
            operators: crate::Operators::Schema::from_builder(builder)?,
            configuration: crate::Configuration::Schema::from_builder(builder)?,
            metaData: crate::MetaData::Schema::from_builder(builder)?,
            typeSchema: crate::Type::Schema::from_builder(builder)?,
            method: crate::Method::Schema::from_builder(builder)?,
            ast: crate::Ast::Schema::from_builder(builder)?,
            callGraph: crate::CallGraph::Schema::from_builder(builder)?,
            cfg: crate::Cfg::Schema::from_builder(builder)?,
            dominators: crate::Dominators::Schema::from_builder(builder)?,
            pdg: crate::Pdg::Schema::from_builder(builder)?,
            shortcuts: crate::Shortcuts::Schema::from_builder(builder)?,
            sourceSpecific: crate::Comment::Schema::from_builder(builder)?,
            tags: crate::Tags::Schema::from_builder(builder)?,
            binding: crate::Binding::Schema::from_builder(builder)?,
            annotation: crate::Annotation::Schema::from_builder(builder)?,
            finding: crate::Finding::Schema::from_builder(builder)?,
            hidden: crate::Hidden::Schema::from_builder(builder)?,
            protoSerialize: crate::ProtoSerialize::Schema::from_builder(builder)?,
        })
    }
}
pub fn instance() -> Result<crate::model::Schema> {
    let bundle: serde_json::Value = serde_json::from_str(DOCUMENT)
        .map_err(|e| crate::Error::new("CpgSchema", e.to_string()))?;
    crate::interpreter::builder_from_bundle(&bundle)?.build()
}
pub mod PropertyDefaults {
    pub fn resolve(name: &str) -> crate::Result<crate::value::Scalar> {
        crate::interpreter::property_default(name)
    }
    pub fn String() -> crate::Result<crate::value::JavaString> {
        if let crate::value::Scalar::String(value) = resolve("String")? {
            Ok(value)
        } else {
            Err(crate::Error::new("String", "default has wrong value type"))
        }
    }
    pub fn Boolean() -> crate::Result<bool> {
        if let crate::value::Scalar::Boolean(value) = resolve("Boolean")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Boolean", "default has wrong value type"))
        }
    }
    pub fn Byte() -> crate::Result<i8> {
        if let crate::value::Scalar::Byte(value) = resolve("Byte")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Byte", "default has wrong value type"))
        }
    }
    pub fn Short() -> crate::Result<i16> {
        if let crate::value::Scalar::Short(value) = resolve("Short")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Short", "default has wrong value type"))
        }
    }
    pub fn Int() -> crate::Result<i32> {
        if let crate::value::Scalar::Int(value) = resolve("Int")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Int", "default has wrong value type"))
        }
    }
    pub fn Long() -> crate::Result<i64> {
        if let crate::value::Scalar::Long(value) = resolve("Long")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Long", "default has wrong value type"))
        }
    }
    pub fn Float() -> crate::Result<u32> {
        if let crate::value::Scalar::Float(value) = resolve("Float")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Float", "default has wrong value type"))
        }
    }
    pub fn Double() -> crate::Result<u64> {
        if let crate::value::Scalar::Double(value) = resolve("Double")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Double", "default has wrong value type"))
        }
    }
    pub fn Char() -> crate::Result<u16> {
        if let crate::value::Scalar::Char(value) = resolve("Char")? {
            Ok(value)
        } else {
            Err(crate::Error::new("Char", "default has wrong value type"))
        }
    }
}
