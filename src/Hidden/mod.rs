//! Native port of Hidden.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, PropertyId, Relation, Result, SchemaBuilder,
};
use crate::value::{Cardinality, EdgeCardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "Hidden";
pub const DOC_INDEX: i32 = -1;
pub const DESCRIPTION: &str = r#"
The schema elements defined here are NOT included on the schema website.
These are schema elements that we are using but would like to rework
in the future and not make part of the standard.
"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub closureBindingId: PropertyId,
    pub capture: EdgeTypeId,
    pub closureBinding: NodeTypeId,
    pub capturedBy: EdgeTypeId,
    pub templateDOM: NodeTypeId,
    pub dependencyGroupId: PropertyId,
    pub dependency: NodeTypeId,
    pub dynamicTypeHintFullName: PropertyId,
    pub possibleTypes: PropertyId,
    pub importNode: NodeTypeId,
    pub importedEntity: PropertyId,
    pub importedAs: PropertyId,
    pub explicitAs: PropertyId,
    pub isWildcard: PropertyId,
    pub isExplicit: PropertyId,
    pub isModuleImport: PropertyId,
    pub imports: EdgeTypeId,
    pub isCallForImport: EdgeTypeId,
    pub secondaryId: PropertyId,
    pub genericSignature: PropertyId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        methodSchema: &crate::Method::Schema,
        typeDeclSchema: &crate::Type::Schema,
        astSchema: &crate::Ast::Schema,
        _cfgSchema: &crate::Cfg::Schema,
        fsSchema: &crate::FileSystem::Schema,
        callGraph: &crate::CallGraph::Schema,
        pdg: &crate::Pdg::Schema,
        tags: &crate::Tags::Schema,
        shortcuts: &crate::Shortcuts::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec![
                "Base".into(),
                "Method".into(),
                "Type".into(),
                "Ast".into(),
                "Cfg".into(),
                "FileSystem".into(),
                "CallGraph".into(),
                "Pdg".into(),
                "Tags".into(),
                "Shortcuts".into(),
            ],
        })?;
        let argument = callGraph.argument;
        let ast = astSchema.ast;
        let astNode = astSchema.astNode;
        let block = astSchema.block;
        let callNode = astSchema.callNode;
        let contains = shortcuts.contains;
        let evaluationStrategy = callGraph.evaluationStrategy;
        let expression = astSchema.expression;
        let file = fsSchema.file;
        let identifier = astSchema.identifier;
        let literal = astSchema.literal;
        let local = astSchema.local;
        let member = typeDeclSchema.member;
        let method = methodSchema.method;
        let methodParameterIn = methodSchema.methodParameterIn;
        let methodRef = astSchema.methodRef;
        let methodReturn = methodSchema.methodReturn;
        let name = base.name;
        let reachingDef = pdg.reachingDef;
        let r#ref = base.r#ref;
        let tag = tags.tag;
        let taggedBy = tags.taggedBy;
        let typeDecl = typeDeclSchema.typeDecl;
        let typeRef = astSchema.typeRef;
        let unknown = astSchema.unknown;
        let version = base.version;
        let closureBindingId = builder.add_property(MODULE_NAME, "CLOSURE_BINDING_ID", ValueType::String, "Identifier which uniquely describes a CLOSURE_BINDING. This property is used to match captured LOCAL nodes with the corresponding CLOSURE_BINDING nodes")?;
        builder.property_mut(closureBindingId)?.proto_id = Some(crate::ProtoIds::ClosureBindingId);
        let capture = builder.add_edge(
            MODULE_NAME,
            "CAPTURE",
            "Represents the capturing of a variable into a closure",
        )?;
        builder.edge_mut(capture)?.proto_id = Some(crate::ProtoIds::Capture);
        builder.add_properties(local, &[closureBindingId])?;
        builder.add_properties(methodParameterIn, &[closureBindingId])?;
        let closureBinding = builder.add_node(
            MODULE_NAME,
            "CLOSURE_BINDING",
            NodeKind::Concrete,
            "Represents the binding of a LOCAL or METHOD_PARAMETER_IN into the closure of a method",
        )?;
        builder.node_mut(closureBinding)?.proto_id = Some(crate::ProtoIds::ClosureBinding);
        builder.add_properties(closureBinding, &[closureBindingId, evaluationStrategy])?;
        let capturedBy = builder.add_edge(
            MODULE_NAME,
            "CAPTURED_BY",
            "Connection between a captured LOCAL and the corresponding CLOSURE_BINDING",
        )?;
        builder.edge_mut(capturedBy)?.proto_id = Some(crate::ProtoIds::CapturedBy);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(local, capturedBy, closureBinding)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodParameterIn, capturedBy, closureBinding)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodRef, capture, closureBinding)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeRef, capture, closureBinding)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            ..Relation::new(closureBinding, r#ref, local)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            ..Relation::new(closureBinding, r#ref, methodParameterIn)
        })?;
        let templateDOM = builder.add_node(
            MODULE_NAME,
            "TEMPLATE_DOM",
            NodeKind::Concrete,
            "This node represents a DOM node used in template languages, e.g., JSX/TSX",
        )?;
        builder.node_mut(templateDOM)?.proto_id = Some(crate::ProtoIds::TemplateDom);
        builder.add_properties(templateDOM, &[name])?;
        builder.extend(templateDOM, &[expression])?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(templateDOM, ast, expression)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(templateDOM, reachingDef, expression)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(templateDOM, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(expression, argument, templateDOM)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(method, contains, templateDOM)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(file, contains, templateDOM)
        })?;
        let dependencyGroupId = builder.add_property(
            MODULE_NAME,
            "DEPENDENCY_GROUP_ID",
            ValueType::String,
            "The group ID for a dependency",
        )?;
        builder.property_mut(dependencyGroupId)?.proto_id =
            Some(crate::ProtoIds::DependencyGroupId);
        let dependency = builder.add_node(
            MODULE_NAME,
            "DEPENDENCY",
            NodeKind::Concrete,
            "This node represents a dependency",
        )?;
        builder.node_mut(dependency)?.proto_id = Some(crate::ProtoIds::Dependency);
        builder.add_properties(dependency, &[version, name, dependencyGroupId])?;
        builder.node_mut(dependency)?.primary_key = Some(name);
        let dynamicTypeHintFullName = builder.add_property(
            MODULE_NAME,
            "DYNAMIC_TYPE_HINT_FULL_NAME",
            ValueType::String,
            "Type hint for the dynamic type. These are observed to be verifiable at runtime.",
        )?;
        builder.property_mut(dynamicTypeHintFullName)?.cardinality = Cardinality::List;
        builder.property_mut(dynamicTypeHintFullName)?.proto_id =
            Some(crate::ProtoIds::DynamicTypeHintFullName);
        let possibleTypes = builder.add_property(MODULE_NAME, "POSSIBLE_TYPES", ValueType::String, "Similar to `DYNAMIC_TYPE_HINT_FULL_NAME`, but that this makes no guarantee that types within this property are correct. This property is used to capture observations between node interactions during a 'may-analysis'.")?;
        builder.property_mut(possibleTypes)?.cardinality = Cardinality::List;
        builder.property_mut(possibleTypes)?.proto_id = Some(crate::ProtoIds::PossibleTypes);
        builder.add_properties(callNode, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(methodParameterIn, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(methodReturn, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(methodRef, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(typeRef, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(local, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(block, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(unknown, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(literal, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(identifier, &[dynamicTypeHintFullName, possibleTypes])?;
        builder.add_properties(member, &[dynamicTypeHintFullName, possibleTypes])?;
        let importNode = builder.add_node(
            MODULE_NAME,
            "IMPORT",
            NodeKind::Concrete,
            r#"Declarative import as it is found in statically typed languages like Java.
This kind of node is not supposed to be used for imports in dynamically typed
languages like Javascript.
"#,
        )?;
        builder.extend(importNode, &[astNode])?;
        builder.node_mut(importNode)?.starter_name = Some("imports".into());
        let importedEntity = builder.add_property(
            MODULE_NAME,
            "IMPORTED_ENTITY",
            ValueType::String,
            r#"The identifying string of the imported entity.
For a Java import like "import java.nio.ByteBuffer;" this would be "java.nio.ByteBuffer".
"#,
        )?;
        let importedAs = builder.add_property(
            MODULE_NAME,
            "IMPORTED_AS",
            ValueType::String,
            r#"The identifier under which the import can be accessed in the importing context.
For a Java import this is always identical to the class name. But e.g. for a
Kotlin import like "import java.nio.ByteBuffer as BBuffer" this would be "BBuffer".
This property is ignored if IS_WILDCARD is true.
"#,
        )?;
        let explicitAs = builder.add_property(
            MODULE_NAME,
            "EXPLICIT_AS",
            ValueType::Boolean,
            r#"Specifies whether the IMPORTED_AS property was explicitly present in the code.
For languages like Java which do not allow a renaming during import this is
always false. For e.g. Kotlin it depends on the existence of the "as" keyword.
"#,
        )?;
        let isWildcard = builder.add_property(
            MODULE_NAME,
            "IS_WILDCARD",
            ValueType::Boolean,
            r#"Specifies whether this is a wildcard import.
For a Java import like "import java.nio.*;" IS_WILDCARD would be "true" and
IMPORTED_ENTITY would be "java.nio".
For wildcard imports the IMPORTED_AS property is ignored.
"#,
        )?;
        let isExplicit = builder.add_property(
            MODULE_NAME,
            "IS_EXPLICIT",
            ValueType::Boolean,
            r#"Specifies whether this is an explicit import.
Most languages have implicit default imports of some standard library elements
and this flag is used to distinguish those from explicit imports found in the
code base.
"#,
        )?;
        let isModuleImport = builder.add_property(
            MODULE_NAME,
            "IS_MODULE_IMPORT",
            ValueType::Boolean,
            r#"Specifies whether this is a module import.
This is used for languages like Java >= 25 where packages exported by a module
can be imported via the module name (which does not need to match the package names in
any way)."#,
        )?;
        builder.add_properties(importNode, &[importedEntity])?;
        builder.add_properties(importNode, &[importedAs])?;
        builder.add_properties(importNode, &[explicitAs])?;
        builder.add_properties(importNode, &[isWildcard])?;
        builder.add_properties(importNode, &[isExplicit])?;
        builder.add_properties(importNode, &[isModuleImport])?;
        let imports =
            builder.add_edge(MODULE_NAME, "IMPORTS", "Edge from imports to dependencies")?;
        builder.edge_mut(imports)?.proto_id = Some(crate::ProtoIds::Imports);
        let isCallForImport = builder.add_edge(
            MODULE_NAME,
            "IS_CALL_FOR_IMPORT",
            r#"Edge from CALL statement in the AST to the IMPORT.
￼        |We use this edge to traverse from the logical representation of the IMPORT
￼        |to the corresponding import statement in the AST.
￼        |"#,
        )?;
        builder.edge_mut(isCallForImport)?.proto_id = Some(crate::ProtoIds::IsCallForImport);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(importNode, imports, dependency)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(importNode, taggedBy, tag)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, isCallForImport, importNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, importNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(file, ast, importNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeDecl, ast, importNode)
        })?;
        let secondaryId = builder.add_property(
            MODULE_NAME,
            "SECONDARY_ID",
            ValueType::String,
            r#"ID from a different context, e.g. if the graph was imported from a different format,
we can use this to preserve the link to the original"#,
        )?;
        let genericSignature = builder.add_property(MODULE_NAME, "GENERIC_SIGNATURE", ValueType::String, r#"This field is experimental. It will likely be removed in the future without any notice.
It stores type information for generic types and methods as well as type information
for members and locals where the type either contains a type parameter reference or
an instantiated type reference.
"#)?;
        builder.property_mut(genericSignature)?.cardinality = Cardinality::One(Some(
            Scalar::string(crate::CpgSchema::PropertyDefaults::String),
        ));
        builder.property_mut(genericSignature)?.proto_id = Some(crate::ProtoIds::GenericSignature);
        builder.add_properties(method, &[genericSignature])?;
        builder.add_properties(typeDecl, &[genericSignature])?;
        builder.add_properties(member, &[genericSignature])?;
        builder.add_properties(local, &[genericSignature])?;
        Ok(Self {
            closureBindingId,
            capture,
            closureBinding,
            capturedBy,
            templateDOM,
            dependencyGroupId,
            dependency,
            dynamicTypeHintFullName,
            possibleTypes,
            importNode,
            importedEntity,
            importedAs,
            explicitAs,
            isWildcard,
            isExplicit,
            isModuleImport,
            imports,
            isCallForImport,
            secondaryId,
            genericSignature,
        })
    }
}
impl crate::SchemaBase::SchemaBase for Schema {
    fn doc_index(&self) -> i32 {
        DOC_INDEX
    }
    fn description(&self) -> &str {
        DESCRIPTION
    }
    fn provided_by_frontend(&self) -> bool {
        PROVIDED_BY_FRONTEND
    }
}
