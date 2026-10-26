//! Native port of Ast.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{
    Constant, ConstantCategoryId, EdgeTypeId, ModuleInfo, NodeKind, NodeTypeId, PropertyId,
    Relation, Result, SchemaBuilder, nonempty,
};
use crate::value::{Cardinality, EdgeCardinality, Scalar, ValueType};
pub const MODULE_NAME: &str = "Ast";
pub const DOC_INDEX: i32 = 7;
pub const DESCRIPTION: &str = r#"
The Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.
All nodes of the tree inherit from the same base class (`AST_NODE`) and are connected
to their child nodes via outgoing `AST` edges.

Syntax trees are typed, that is, when possible, types for all expressions are stored
in the tree. Moreover, common control structure types are defined in the specification,
making it possible to translate trees into corresponding control flow graphs if only
these common control structure types are used, possibly by desugaring on the side of
the language frontend. For cases where this is not an option,
the AST specification provides means of storing language-dependent information in the
AST that can be interpreted by language-dependent control flow construction passes.

This layer MUST be created by the frontend.

"#;
pub const PROVIDED_BY_FRONTEND: bool = true;
#[derive(Clone, Debug)]
pub struct Schema {
    pub order: PropertyId,
    pub astNode: NodeTypeId,
    pub block: NodeTypeId,
    pub literal: NodeTypeId,
    pub local: NodeTypeId,
    pub identifier: NodeTypeId,
    pub canonicalName: PropertyId,
    pub fieldIdentifier: NodeTypeId,
    pub modifierType: PropertyId,
    pub modifierTypes: ConstantCategoryId,
    pub modifier: NodeTypeId,
    pub jumpTarget: NodeTypeId,
    pub jumpLabel: NodeTypeId,
    pub methodRef: NodeTypeId,
    pub typeRef: NodeTypeId,
    pub ret: NodeTypeId,
    pub controlStructureType: PropertyId,
    pub controlStructureTypes: ConstantCategoryId,
    pub controlStructure: NodeTypeId,
    pub unknown: NodeTypeId,
    pub ast: EdgeTypeId,
    pub condition: EdgeTypeId,
    pub trueBody: EdgeTypeId,
    pub falseBody: EdgeTypeId,
    pub doBody: EdgeTypeId,
    pub forInit: EdgeTypeId,
    pub forUpdate: EdgeTypeId,
    pub forBody: EdgeTypeId,
    pub tryBody: EdgeTypeId,
    pub catchBody: EdgeTypeId,
    pub finallyBody: EdgeTypeId,
    pub jumpArgument: EdgeTypeId,
    pub callRepr: NodeTypeId,
    pub callNode: NodeTypeId,
    pub expression: NodeTypeId,
}
impl Schema {
    pub fn new(
        builder: &mut SchemaBuilder,
        base: &crate::Base::Schema,
        namespaces: &crate::Namespace::Schema,
        methodSchema: &crate::Method::Schema,
        typeSchema: &crate::Type::Schema,
        fs: &crate::FileSystem::Schema,
    ) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec![
                "Base".into(),
                "Namespace".into(),
                "Method".into(),
                "Type".into(),
                "FileSystem".into(),
            ],
        })?;
        let code = base.code;
        let columnNumber = fs.columnNumber;
        let declaration = base.declaration;
        let file = fs.file;
        let lineNumber = fs.lineNumber;
        let member = typeSchema.member;
        let method = methodSchema.method;
        let methodParameterIn = methodSchema.methodParameterIn;
        let methodParameterOut = methodSchema.methodParameterOut;
        let methodReturn = methodSchema.methodReturn;
        let name = base.name;
        let namespace = namespaces.namespace;
        let namespaceBlock = namespaces.namespaceBlock;
        let offset = fs.offset;
        let offsetEnd = fs.offsetEnd;
        let parserTypeName = base.parserTypeName;
        let r#ref = base.r#ref;
        let signature = methodSchema.signature;
        let tpe = typeSchema.tpe;
        let typeArgument = typeSchema.typeArgument;
        let typeDecl = typeSchema.typeDecl;
        let typeFullName = typeSchema.typeFullName;
        let typeParameter = typeSchema.typeParameter;
        let order = builder.add_property(
            MODULE_NAME,
            "ORDER",
            ValueType::Int,
            r#"This integer indicates the position of the node among
its siblings in the AST. The left-most child has an
order of 0.
"#,
        )?;
        builder.property_mut(order)?.cardinality = Cardinality::One(Some(Scalar::Int(-1)));
        builder.property_mut(order)?.proto_id = Some(crate::ProtoIds::Order);
        let astNode = builder.add_node(
            MODULE_NAME,
            "AST_NODE",
            NodeKind::Base,
            r#"This is the base type for all nodes of the abstract syntax tree (AST). An AST
node has a `CODE` and an `ORDER` field. The `CODE` field contains the
code (verbatim) represented by the AST node. The `ORDER` field contains the
nodes position among its siblings, encoded as an integer where the left most
sibling has the position `0`.

AST nodes contain optional `LINE_NUMBER` and `COLUMN_NUMBER` fields. For
source-based frontends, these fields contain the start line number and
start column number of the code represented by the node.
For machine-code-based and bytecode-based frontends, `LINE_NUMBER` contains
the address at which the code starts while `COLUMN_NUMBER` is undefined.

The optional `OFFSET` and `OFFSET_END` specify the start
and exclusive end position of the code belonging to a node within the corresponding
`FILE` nodes `CONTENT` property.
"#,
        )?;
        builder.add_properties(astNode, &[order, code])?;
        builder.add_properties(astNode, &[lineNumber, columnNumber, offset, offsetEnd])?;
        builder.extend(method, &[astNode])?;
        builder.extend(methodParameterIn, &[astNode])?;
        builder.extend(methodParameterOut, &[astNode])?;
        builder.extend(typeDecl, &[astNode])?;
        builder.extend(member, &[astNode])?;
        builder.extend(typeParameter, &[astNode])?;
        builder.extend(typeArgument, &[astNode])?;
        builder.extend(file, &[astNode])?;
        builder.extend(namespaceBlock, &[astNode])?;
        let block = builder.add_node(MODULE_NAME, "BLOCK", NodeKind::Concrete, r#"This node represents a compound statement. Compound statements are used in many languages to allow
grouping a sequence of statements. For example, in C and Java, compound statements
are statements enclosed by curly braces. Function/Method bodies are compound
statements. We do not use the term "compound statement" because "statement" would
imply that the block does not yield a value upon evaluation, that is, that it is
not an expression. This is true in languages such as C and Java, but not for languages
such as Scala where the value of the block is given by that of the last expression it
contains. In fact, the Scala grammar uses the term "BlockExpr" (short for
"block expression") to describe what in the CPG we call "Block".
"#)?;
        builder.node_mut(block)?.proto_id = Some(crate::ProtoIds::Block);
        builder.add_properties(block, &[typeFullName])?;
        let literal = builder.add_node(
            MODULE_NAME,
            "LITERAL",
            NodeKind::Concrete,
            r#"This node represents a literal such as an integer or string constant. Literals
are symbols included in the code in verbatim form and which are immutable.
The `TYPE_FULL_NAME` field stores the literal's fully-qualified type name,
e.g., `java.lang.Integer`.
"#,
        )?;
        builder.node_mut(literal)?.proto_id = Some(crate::ProtoIds::Literal);
        builder.add_properties(literal, &[typeFullName])?;
        builder.node_mut(literal)?.primary_key = Some(code);
        let local = builder.add_node(
            MODULE_NAME,
            "LOCAL",
            NodeKind::Concrete,
            r#"This node represents a local variable. Its fully qualified type name is stored
in the `TYPE_FULL_NAME` field and its name in the `NAME` field. The `CODE` field
contains the entire local variable declaration without initialization, e.g., for
`int x = 10;`, it contains `int x`.
"#,
        )?;
        builder.node_mut(local)?.proto_id = Some(crate::ProtoIds::Local);
        builder.add_properties(local, &[typeFullName])?;
        builder.extend(local, &[declaration, astNode])?;
        builder.node_mut(local)?.primary_key = Some(name);
        let identifier = builder.add_node(
            MODULE_NAME,
            "IDENTIFIER",
            NodeKind::Concrete,
            r#"This node represents an identifier as used when referring to a variable by name.
It holds the identifier's name in the `NAME` field and its fully-qualified type
name in `TYPE_FULL_NAME`.
"#,
        )?;
        builder.node_mut(identifier)?.proto_id = Some(crate::ProtoIds::Identifier);
        builder.add_properties(identifier, &[typeFullName, name])?;
        builder.node_mut(identifier)?.primary_key = Some(name);
        let canonicalName = builder.add_property(
            MODULE_NAME,
            "CANONICAL_NAME",
            ValueType::String,
            r#"This field holds the canonical name of a `FIELD_IDENTIFIER`. It is typically
identical to the CODE field, but canonicalized according to source language
semantics. Human readable names are preferable. `FIELD_IDENTIFIER` nodes must
share identical `CANONICAL_NAME` if and
only if they alias, e.g., in C-style unions (if the aliasing relationship is
unknown or there are partial overlaps, then one must make a reasonable guess,
and trade off between false negatives and false positives).
"#,
        )?;
        builder.property_mut(canonicalName)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(canonicalName)?.proto_id = Some(crate::ProtoIds::CanonicalName);
        let fieldIdentifier = builder.add_node(
            MODULE_NAME,
            "FIELD_IDENTIFIER",
            NodeKind::Concrete,
            r#"This node represents the field accessed in a field access, e.g., in
`a.b`, it represents `b`. The field name as it occurs in the code is
stored in the `CODE` field. This may mean that the `CODE` field holds
an expression. The `CANONICAL_NAME` field MAY contain the same value is
the `CODE` field but SHOULD contain the normalized name that results
from evaluating `CODE` as an expression if such an evaluation is
possible for the language frontend. The objective is to store an identifier
in `CANONICAL_NAME` that is the same for two nodes iff they refer to the
same field, regardless of whether they use the same expression to reference
it.
"#,
        )?;
        builder.node_mut(fieldIdentifier)?.proto_id = Some(crate::ProtoIds::FieldIdentifier);
        builder.add_properties(fieldIdentifier, &[canonicalName])?;
        let modifierType = builder.add_property(
            MODULE_NAME,
            "MODIFIER_TYPE",
            ValueType::String,
            r#"The modifier type is a free-form string. The following are known modifier types:
`STATIC`, `PUBLIC`, `PROTECTED`, `PRIVATE`, `ABSTRACT`, `NATIVE`, `CONSTRUCTOR`, `VIRTUAL`.
"#,
        )?;
        builder.property_mut(modifierType)?.cardinality = Cardinality::One(Some(Scalar::string(
            crate::CpgSchema::PropertyDefaults::String,
        )));
        builder.property_mut(modifierType)?.proto_id = Some(crate::ProtoIds::ModifierType);
        let modifierTypes = builder.add_constants("ModifierTypes", vec![
    Constant::new("STATIC", "STATIC", ValueType::String, "The static modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Static),
    Constant::new("PUBLIC", "PUBLIC", ValueType::String, "The public modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Public),
    Constant::new("PROTECTED", "PROTECTED", ValueType::String, "The protected modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Protected),
    Constant::new("PRIVATE", "PRIVATE", ValueType::String, "The private modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Private),
    Constant::new("ABSTRACT", "ABSTRACT", ValueType::String, "The abstract modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Abstract),
    Constant::new("NATIVE", "NATIVE", ValueType::String, "The native modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Native),
    Constant::new("CONSTRUCTOR", "CONSTRUCTOR", ValueType::String, "The constructor modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Constructor),
    Constant::new("VIRTUAL", "VIRTUAL", ValueType::String, "The virtual modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Virtual),
    Constant::new("INTERNAL", "INTERNAL", ValueType::String, "The internal modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Internal),
    Constant::new("FINAL", "FINAL", ValueType::String, "The final modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::Final),
    Constant::new("READONLY", "READONLY", ValueType::String, "The readonly modifier", MODULE_NAME).with_proto_id(crate::ProtoIds::ReadOnly),
    Constant::new("MODULE", "MODULE", ValueType::String, "Indicate that a method defines a module in the sense e.g. a python module does with the creation of a module object", MODULE_NAME).with_proto_id(crate::ProtoIds::Module),
    Constant::new("LAMBDA", "LAMBDA", ValueType::String, "Indicate that a method is an anonymous function, lambda, or closure", MODULE_NAME).with_proto_id(crate::ProtoIds::Lambda)
])?;
        let modifier = builder.add_node(
            MODULE_NAME,
            "MODIFIER",
            NodeKind::Concrete,
            r#"This field represents a (language-dependent) modifier such as `static`, `private`
or `public`. Unlike most other AST nodes, it is NOT an expression, that is, it
cannot be evaluated and cannot be passed as an argument in function calls.
"#,
        )?;
        builder.node_mut(modifier)?.proto_id = Some(crate::ProtoIds::Modifier);
        builder.add_properties(modifier, &[modifierType])?;
        builder.extend(modifier, &[astNode])?;
        let jumpTarget = builder.add_node(
            MODULE_NAME,
            "JUMP_TARGET",
            NodeKind::Concrete,
            r#"A jump target is any location in the code that has been specifically marked
as the target of a jump, e.g., via a label. The `NAME` field holds the name of
the label while the `PARSER_TYPE_NAME` field holds the name of language construct
that this jump target is created from, e.g., "Label".
"#,
        )?;
        builder.node_mut(jumpTarget)?.proto_id = Some(crate::ProtoIds::JumpTarget);
        builder.add_properties(jumpTarget, &[name, parserTypeName])?;
        builder.extend(jumpTarget, &[astNode])?;
        let jumpLabel = builder.add_node(
            MODULE_NAME,
            "JUMP_LABEL",
            NodeKind::Concrete,
            r#"A jump label specifies the label and thus the JUMP_TARGET of control structures
BREAK and CONTINUE. The `NAME` field holds the name of the label while the
`PARSER_TYPE_NAME` field holds the name of language construct that this jump
label is created from, e.g., "Label".
"#,
        )?;
        builder.node_mut(jumpLabel)?.proto_id = Some(crate::ProtoIds::JumpLabel);
        builder.add_properties(jumpLabel, &[name, parserTypeName])?;
        builder.extend(jumpLabel, &[astNode])?;
        let methodRef = builder.add_node(
            MODULE_NAME,
            "METHOD_REF",
            NodeKind::Concrete,
            r#"This node represents a reference to a method/function/procedure as it
appears when a method is passed as an argument in a call. The `METHOD_FULL_NAME`
field holds the fully-qualified name of the referenced method and the
`TYPE_FULL_NAME` holds its fully-qualified type name.
"#,
        )?;
        builder.node_mut(methodRef)?.proto_id = Some(crate::ProtoIds::MethodRef);
        builder.add_properties(methodRef, &[typeFullName])?;
        let typeRef = builder.add_node(
            MODULE_NAME,
            "TYPE_REF",
            NodeKind::Concrete,
            "Reference to a type/class",
        )?;
        builder.node_mut(typeRef)?.proto_id = Some(crate::ProtoIds::TypeRef);
        builder.add_properties(typeRef, &[typeFullName])?;
        let ret = builder.add_node(
            MODULE_NAME,
            "RETURN",
            NodeKind::Concrete,
            r#"This node represents a return instruction, e.g., `return x`. Note that it does
NOT represent a formal return parameter as formal return parameters are
represented via `METHOD_RETURN` nodes.
"#,
        )?;
        builder.node_mut(ret)?.proto_id = Some(crate::ProtoIds::Return);
        builder.node_mut(ret)?.starter_name = Some("ret".into());
        builder.node_mut(ret)?.primary_key = Some(code);
        let controlStructureType = builder.add_property(
            MODULE_NAME,
            "CONTROL_STRUCTURE_TYPE",
            ValueType::String,
            r#"The `CONTROL_STRUCTURE_TYPE` field indicates which kind of control structure
a `CONTROL_STRUCTURE` node represents. The available types are the following:
 BREAK, CONTINUE, DO, WHILE, FOR, GOTO, IF, ELSE, TRY, THROW and SWITCH.
"#,
        )?;
        builder.property_mut(controlStructureType)?.cardinality = Cardinality::One(Some(
            Scalar::string(crate::CpgSchema::PropertyDefaults::String),
        ));
        builder.property_mut(controlStructureType)?.proto_id =
            Some(crate::ProtoIds::ControlStructureType);
        let controlStructureTypes = builder.add_constants("ControlStructureTypes", vec![
    Constant::new("BREAK", "BREAK", ValueType::String, r#"Represents a break statement. Labeled breaks are expected to have a JUMP_LABEL
node AST child with ORDER 1"#, MODULE_NAME).with_proto_id(crate::ProtoIds::Break),
    Constant::new("CONTINUE", "CONTINUE", ValueType::String, r#"Represents a continue statement. Labeled continues are expected to have a JUMP_LABEL
node AST child with ORDER 1"#, MODULE_NAME).with_proto_id(crate::ProtoIds::Continue),
    Constant::new("WHILE", "WHILE", ValueType::String, "Represents a while statement", MODULE_NAME).with_proto_id(crate::ProtoIds::While),
    Constant::new("DO", "DO", ValueType::String, "Represents a do statement", MODULE_NAME).with_proto_id(crate::ProtoIds::Do),
    Constant::new("FOR", "FOR", ValueType::String, "Represents a for statement", MODULE_NAME).with_proto_id(crate::ProtoIds::For),
    Constant::new("GOTO", "GOTO", ValueType::String, "Represents a goto statement", MODULE_NAME).with_proto_id(crate::ProtoIds::Goto),
    Constant::new("IF", "IF", ValueType::String, "Represents an if statement", MODULE_NAME).with_proto_id(crate::ProtoIds::If),
    Constant::new("ELSE", "ELSE", ValueType::String, "Represents an else statement", MODULE_NAME).with_proto_id(crate::ProtoIds::Else),
    Constant::new("SWITCH", "SWITCH", ValueType::String, "Represents a switch statement", MODULE_NAME).with_proto_id(crate::ProtoIds::Switch),
    Constant::new("TRY", "TRY", ValueType::String, "Represents a try statement", MODULE_NAME).with_proto_id(crate::ProtoIds::Try),
    Constant::new("THROW", "THROW", ValueType::String, "Represents a throw statement", MODULE_NAME).with_proto_id(crate::ProtoIds::Throw),
    Constant::new("MATCH", "MATCH", ValueType::String, "Represents a match expression", MODULE_NAME).with_proto_id(crate::ProtoIds::Match),
    Constant::new("YIELD", "YIELD", ValueType::String, "Represents a yield expression", MODULE_NAME).with_proto_id(crate::ProtoIds::Yield),
    Constant::new("CATCH", "CATCH", ValueType::String, "Represents a catch clause", MODULE_NAME).with_proto_id(crate::ProtoIds::Catch),
    Constant::new("FINALLY", "FINALLY", ValueType::String, "Represents a finally clause", MODULE_NAME).with_proto_id(crate::ProtoIds::Finally)
])?;
        let controlStructure = builder.add_node(
            MODULE_NAME,
            "CONTROL_STRUCTURE",
            NodeKind::Concrete,
            r#"This node represents a control structure as introduced by control structure
statements as well as conditional and unconditional jumps. Its type is stored in the
`CONTROL_STRUCTURE_TYPE` field to be one of several pre-defined types. These types
 are used in the construction of the control flow layer, making it possible to
 generate the control flow layer from the abstract syntax tree layer automatically.

In addition to the `CONTROL_STRUCTURE_TYPE` field, the `PARSER_TYPE_NAME` field
MAY be used by frontends to store the name of the control structure as emitted by
the parser or disassembler, however, the value of this field is not relevant
for construction of the control flow layer.
"#,
        )?;
        builder.node_mut(controlStructure)?.proto_id = Some(crate::ProtoIds::ControlStructure);
        builder.add_properties(controlStructure, &[parserTypeName, controlStructureType])?;
        let unknown = builder.add_node(
            MODULE_NAME,
            "UNKNOWN",
            NodeKind::Concrete,
            r#"Any AST node that the frontend would like to include in the AST but for
which no suitable AST node is specified in the CPG specification may be
included using a node of type `UNKNOWN`.
"#,
        )?;
        builder.node_mut(unknown)?.proto_id = Some(crate::ProtoIds::Unknown);
        builder.add_properties(unknown, &[parserTypeName, typeFullName])?;
        let ast = builder.add_edge(
            MODULE_NAME,
            "AST",
            "This edge connects a parent node to its child in the syntax tree.",
        )?;
        builder.edge_mut(ast)?.proto_id = Some(crate::ProtoIds::Ast);
        let condition = builder.add_edge(MODULE_NAME, "CONDITION", "The edge connects control structure nodes to the expressions that holds their conditions.")?;
        builder.edge_mut(condition)?.proto_id = Some(crate::ProtoIds::Condition);
        let trueBody = builder.add_edge(
            MODULE_NAME,
            "TRUE_BODY",
            "The edge connects control structure nodes to their true branch/body.",
        )?;
        builder.edge_mut(trueBody)?.proto_id = Some(crate::ProtoIds::TrueBody);
        let falseBody = builder.add_edge(
            MODULE_NAME,
            "FALSE_BODY",
            "The edge connects control structure nodes to their false branch/body.",
        )?;
        builder.edge_mut(falseBody)?.proto_id = Some(crate::ProtoIds::FalseBody);
        let doBody = builder.add_edge(
            MODULE_NAME,
            "DO_BODY",
            "The edge connects do-while control structure nodes to their body.",
        )?;
        builder.edge_mut(doBody)?.proto_id = Some(crate::ProtoIds::DoBody);
        let forInit = builder.add_edge(MODULE_NAME, "FOR_INIT", "The edge connects for-loop control structure nodes to their initialization expression(s).")?;
        builder.edge_mut(forInit)?.proto_id = Some(crate::ProtoIds::ForInit);
        let forUpdate = builder.add_edge(MODULE_NAME, "FOR_UPDATE", "The edge connects for-loop control structure nodes to their update/step expression(s).")?;
        builder.edge_mut(forUpdate)?.proto_id = Some(crate::ProtoIds::ForUpdate);
        let forBody = builder.add_edge(
            MODULE_NAME,
            "FOR_BODY",
            "The edge connects for-loop control structure nodes to their body.",
        )?;
        builder.edge_mut(forBody)?.proto_id = Some(crate::ProtoIds::ForBody);
        let tryBody = builder.add_edge(
            MODULE_NAME,
            "TRY_BODY",
            "The edge connects try control structure nodes to their try body.",
        )?;
        builder.edge_mut(tryBody)?.proto_id = Some(crate::ProtoIds::TryBody);
        let catchBody = builder.add_edge(
            MODULE_NAME,
            "CATCH_BODY",
            "The edge connects try control structure nodes to catch/handler bodies.",
        )?;
        builder.edge_mut(catchBody)?.proto_id = Some(crate::ProtoIds::CatchBody);
        let finallyBody = builder.add_edge(
            MODULE_NAME,
            "FINALLY_BODY",
            "The edge connects try control structure nodes to their finally body.",
        )?;
        builder.edge_mut(finallyBody)?.proto_id = Some(crate::ProtoIds::FinallyBody);
        let jumpArgument = builder.add_edge(MODULE_NAME, "JUMP_ARGUMENT", "The edge connects jump-like control structures to the node encoding their jump target.")?;
        builder.edge_mut(jumpArgument)?.proto_id = Some(crate::ProtoIds::JumpArgument);
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(file, ast, namespaceBlock)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(member, ast, modifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(tpe, ast, typeArgument)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(typeDecl, ast, typeParameter)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            step_name_in: nonempty("typeDecl"),
            step_doc_in: nonempty("The type declaration this member is defined in"),
            ..Relation::new(typeDecl, ast, member)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(typeDecl, ast, modifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::One,
            cardinality_in: EdgeCardinality::One,
            step_name_out: nonempty("methodReturn"),
            step_doc_out: nonempty("Formal return parameters"),
            ..Relation::new(method, ast, methodReturn)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            step_name_out: nonempty("parameter"),
            step_doc_out: nonempty("Parameters of the method"),
            step_name_in: nonempty("method"),
            step_doc_in: nonempty("Traverse to method associated with this formal parameter"),
            ..Relation::new(method, ast, methodParameterIn)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(method, ast, modifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::One,
            cardinality_in: EdgeCardinality::One,
            step_name_out: nonempty("block"),
            step_doc_out: nonempty("Root of the abstract syntax tree"),
            ..Relation::new(method, ast, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(method, ast, typeParameter)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(block, ast, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            step_name_in: nonempty("definingBlock"),
            step_doc_in: nonempty("The block in which local is declared."),
            step_name_out: nonempty("local"),
            step_doc_out: nonempty("Traverse to locals of this block."),
            ..Relation::new(block, ast, local)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(controlStructure, ast, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, ast, modifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, ast, local)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(controlStructure, ast, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(controlStructure, ast, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(controlStructure, ast, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, ast, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, ast, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, ast, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(controlStructure, ast, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, ast, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, ast, jumpLabel)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, modifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, local)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, fieldIdentifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, member)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(methodParameterIn, ast, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            step_name_in: nonempty("namespaceBlock"),
            ..Relation::new(namespaceBlock, ast, typeDecl)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(namespaceBlock, ast, method)
        })?;
        builder.extend(namespace, &[astNode])?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, jumpTarget)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, unknown)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, jumpArgument, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, jumpArgument, jumpLabel)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(typeDecl, ast, typeDecl)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(typeDecl, ast, method)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            step_name_in: nonempty("method"),
            ..Relation::new(method, ast, methodParameterOut)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(method, ast, typeDecl)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::ZeroOrOne,
            ..Relation::new(method, ast, method)
        })?;
        let callRepr = builder.add_node(
            MODULE_NAME,
            "CALL_REPR",
            NodeKind::Base,
            "This is the base class of `CALL` that language implementers may safely ignore.",
        )?;
        builder.add_properties(callRepr, &[name, signature])?;
        let callNode = builder.add_node(MODULE_NAME, "CALL", NodeKind::Concrete, r#"A (function/method/procedure) call. The `METHOD_FULL_NAME` property is the name of the
invoked method (the callee) while the `TYPE_FULL_NAME` is its return type, and
therefore, the return type of the call when viewing it as an expression. For
languages like Javascript, it is common that we may know the (short-) name
of the invoked method, but we do not know at compile time which method
will actually be invoked, e.g., because it depends on a dynamic import.
In this case, we leave `METHOD_FULL_NAME` blank but at least fill out `NAME`,
which contains the method's (short-) name and `SIGNATURE`, which contains
any information we may have about the types of arguments and return value.
"#)?;
        builder.node_mut(callNode)?.proto_id = Some(crate::ProtoIds::CallNode);
        builder.extend(callNode, &[callRepr])?;
        builder.add_properties(callNode, &[typeFullName])?;
        builder.node_mut(callNode)?.primary_key = Some(name);
        let expression = builder.add_node(
            MODULE_NAME,
            "EXPRESSION",
            NodeKind::Base,
            r#"`EXPRESSION` is the base class for all nodes that represent code pieces
that can be evaluated.

 Expression may be arguments in method calls. For method calls that do
 not involved named parameters, the `ARGUMENT_INDEX` field indicates at
 which position in the argument list the expression occurs, e.g., an
 `ARGUMENT_INDEX` of 1 indicates that the expression is the first argument
 in a method call. For calls that employ named parameters, `ARGUMENT_INDEX`
 is set to -1 and the `ARGUMENT_NAME` fields holds the name of the parameter.
"#,
        )?;
        builder.extend(expression, &[astNode])?;
        builder.extend(fieldIdentifier, &[expression])?;
        builder.extend(identifier, &[expression])?;
        builder.extend(literal, &[expression])?;
        builder.extend(block, &[expression])?;
        builder.extend(controlStructure, &[expression])?;
        builder.extend(methodRef, &[expression])?;
        builder.extend(typeRef, &[expression])?;
        builder.extend(ret, &[expression])?;
        builder.extend(unknown, &[expression])?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, identifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(callNode, ast, fieldIdentifier)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, literal)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, methodRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, typeRef)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, ret)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, block)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(callNode, ast, controlStructure)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(ret, ast, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_in: EdgeCardinality::One,
            ..Relation::new(controlStructure, ast, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(unknown, ast, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, condition, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, trueBody, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, falseBody, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, doBody, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forInit, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forUpdate, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, forBody, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, tryBody, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, catchBody, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(controlStructure, finallyBody, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(block, ast, callNode)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            step_name_in: nonempty("referencingIdentifiers"),
            step_doc_in: nonempty("Places (identifier) where this local is being referenced"),
            ..Relation::new(identifier, r#ref, local)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            cardinality_out: EdgeCardinality::ZeroOrOne,
            step_name_in: nonempty("referencingIdentifiers"),
            step_doc_in: nonempty("Places (identifier) where this parameter is being referenced"),
            ..Relation::new(identifier, r#ref, methodParameterIn)
        })?;
        builder.add_relation(Relation {
            module: MODULE_NAME.into(),
            ..Relation::new(namespaceBlock, r#ref, namespace)
        })?;
        Ok(Self {
            order,
            astNode,
            block,
            literal,
            local,
            identifier,
            canonicalName,
            fieldIdentifier,
            modifierType,
            modifierTypes,
            modifier,
            jumpTarget,
            jumpLabel,
            methodRef,
            typeRef,
            ret,
            controlStructureType,
            controlStructureTypes,
            controlStructure,
            unknown,
            ast,
            condition,
            trueBody,
            falseBody,
            doBody,
            forInit,
            forUpdate,
            forBody,
            tryBody,
            catchBody,
            finallyBody,
            jumpArgument,
            callRepr,
            callNode,
            expression,
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
