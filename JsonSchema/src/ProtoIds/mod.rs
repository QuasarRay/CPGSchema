//! JSON-owned named protocol identifiers. Namespaces remain defined by their use.
pub const DOCUMENT: &str = include_str!("schema.json");
pub fn resolve(name: &str) -> crate::Result<i32> {
    let document: serde_json::Value =
        serde_json::from_str(DOCUMENT).map_err(|e| crate::Error::new("ProtoIds", e.to_string()))?;
    document
        .get("cpg:protoIds")
        .and_then(|v| v.get(name))
        .and_then(serde_json::Value::as_i64)
        .and_then(|v| i32::try_from(v).ok())
        .ok_or_else(|| crate::Error::new(name, "unknown or invalid protocol identifier"))
}
pub fn Break() -> crate::Result<i32> {
    resolve("Break")
}
pub fn Static() -> crate::Result<i32> {
    resolve("Static")
}
pub fn ByReference() -> crate::Result<i32> {
    resolve("ByReference")
}
pub fn StaticDispatch() -> crate::Result<i32> {
    resolve("StaticDispatch")
}
pub fn Java() -> crate::Result<i32> {
    resolve("Java")
}
pub fn Method() -> crate::Result<i32> {
    resolve("Method")
}
pub fn Public() -> crate::Result<i32> {
    resolve("Public")
}
pub fn BySharing() -> crate::Result<i32> {
    resolve("BySharing")
}
pub fn Continue() -> crate::Result<i32> {
    resolve("Continue")
}
pub fn DynamicDispatch() -> crate::Result<i32> {
    resolve("DynamicDispatch")
}
pub fn LineNumber() -> crate::Result<i32> {
    resolve("LineNumber")
}
pub fn JavaScript() -> crate::Result<i32> {
    resolve("JavaScript")
}
pub fn Ast() -> crate::Result<i32> {
    resolve("Ast")
}
pub fn Protected() -> crate::Result<i32> {
    resolve("Protected")
}
pub fn While() -> crate::Result<i32> {
    resolve("While")
}
pub fn ParserTypeName() -> crate::Result<i32> {
    resolve("ParserTypeName")
}
pub fn ByValue() -> crate::Result<i32> {
    resolve("ByValue")
}
pub fn Inlined() -> crate::Result<i32> {
    resolve("Inlined")
}
pub fn Golang() -> crate::Result<i32> {
    resolve("Golang")
}
pub fn MethodReturn() -> crate::Result<i32> {
    resolve("MethodReturn")
}
pub fn Private() -> crate::Result<i32> {
    resolve("Private")
}
pub fn Order() -> crate::Result<i32> {
    resolve("Order")
}
pub fn Do() -> crate::Result<i32> {
    resolve("Do")
}
pub fn CSharp() -> crate::Result<i32> {
    resolve("CSharp")
}
pub fn Abstract() -> crate::Result<i32> {
    resolve("Abstract")
}
pub fn Annotation() -> crate::Result<i32> {
    resolve("Annotation")
}
pub fn For() -> crate::Result<i32> {
    resolve("For")
}
pub fn Name() -> crate::Result<i32> {
    resolve("Name")
}
pub fn C() -> crate::Result<i32> {
    resolve("C")
}
pub fn FullName() -> crate::Result<i32> {
    resolve("FullName")
}
pub fn AnnotationParameterAssign() -> crate::Result<i32> {
    resolve("AnnotationParameterAssign")
}
pub fn Goto() -> crate::Result<i32> {
    resolve("Goto")
}
pub fn Native() -> crate::Result<i32> {
    resolve("Native")
}
pub fn CallEdge() -> crate::Result<i32> {
    resolve("CallEdge")
}
pub fn Python() -> crate::Result<i32> {
    resolve("Python")
}
pub fn AnnotationParameter() -> crate::Result<i32> {
    resolve("AnnotationParameter")
}
pub fn If() -> crate::Result<i32> {
    resolve("If")
}
pub fn Constructor() -> crate::Result<i32> {
    resolve("Constructor")
}
pub fn IsExternal() -> crate::Result<i32> {
    resolve("IsExternal")
}
pub fn LLVM() -> crate::Result<i32> {
    resolve("LLVM")
}
pub fn Virtual() -> crate::Result<i32> {
    resolve("Virtual")
}
pub fn Value() -> crate::Result<i32> {
    resolve("Value")
}
pub fn Else() -> crate::Result<i32> {
    resolve("Else")
}
pub fn Literal() -> crate::Result<i32> {
    resolve("Literal")
}
pub fn PHP() -> crate::Result<i32> {
    resolve("PHP")
}
pub fn Internal() -> crate::Result<i32> {
    resolve("Internal")
}
pub fn Switch() -> crate::Result<i32> {
    resolve("Switch")
}
pub fn FuzzyTestLang() -> crate::Result<i32> {
    resolve("FuzzyTestLang")
}
pub fn Member() -> crate::Result<i32> {
    resolve("Member")
}
pub fn Final() -> crate::Result<i32> {
    resolve("Final")
}
pub fn Try() -> crate::Result<i32> {
    resolve("Try")
}
pub fn Ref() -> crate::Result<i32> {
    resolve("Ref")
}
pub fn Ghidra() -> crate::Result<i32> {
    resolve("Ghidra")
}
pub fn Kotlin() -> crate::Result<i32> {
    resolve("Kotlin")
}
pub fn ReadOnly() -> crate::Result<i32> {
    resolve("ReadOnly")
}
pub fn Throw() -> crate::Result<i32> {
    resolve("Throw")
}
pub fn ColumnNumber() -> crate::Result<i32> {
    resolve("ColumnNumber")
}
pub fn Variable() -> crate::Result<i32> {
    resolve("Variable")
}
pub fn TaggedBy() -> crate::Result<i32> {
    resolve("TaggedBy")
}
pub fn NewC() -> crate::Result<i32> {
    resolve("NewC")
}
pub fn LineNumberEnd() -> crate::Result<i32> {
    resolve("LineNumberEnd")
}
pub fn Module() -> crate::Result<i32> {
    resolve("Module")
}
pub fn Match() -> crate::Result<i32> {
    resolve("Match")
}
pub fn ParameterLink() -> crate::Result<i32> {
    resolve("ParameterLink")
}
pub fn JavaSrc() -> crate::Result<i32> {
    resolve("JavaSrc")
}
pub fn Lambda() -> crate::Result<i32> {
    resolve("Lambda")
}
pub fn Version() -> crate::Result<i32> {
    resolve("Version")
}
pub fn Yield() -> crate::Result<i32> {
    resolve("Yield")
}
pub fn PythonSrc() -> crate::Result<i32> {
    resolve("PythonSrc")
}
pub fn ArrayInitializer() -> crate::Result<i32> {
    resolve("ArrayInitializer")
}
pub fn Catch() -> crate::Result<i32> {
    resolve("Catch")
}
pub fn Finally() -> crate::Result<i32> {
    resolve("Finally")
}
pub fn CallNode() -> crate::Result<i32> {
    resolve("CallNode")
}
pub fn EvaluationStrategy() -> crate::Result<i32> {
    resolve("EvaluationStrategy")
}
pub fn JsSrc() -> crate::Result<i32> {
    resolve("JsSrc")
}
pub fn ColumnNumberEnd() -> crate::Result<i32> {
    resolve("ColumnNumberEnd")
}
pub fn RubySrc() -> crate::Result<i32> {
    resolve("RubySrc")
}
pub fn SwiftSrc() -> crate::Result<i32> {
    resolve("SwiftSrc")
}
pub fn Cfg() -> crate::Result<i32> {
    resolve("Cfg")
}
pub fn Language() -> crate::Result<i32> {
    resolve("Language")
}
pub fn CSharpSrc() -> crate::Result<i32> {
    resolve("CSharpSrc")
}
pub fn ABAP() -> crate::Result<i32> {
    resolve("ABAP")
}
pub fn Rust() -> crate::Result<i32> {
    resolve("Rust")
}
pub fn Content() -> crate::Result<i32> {
    resolve("Content")
}
pub fn Code() -> crate::Result<i32> {
    resolve("Code")
}
pub fn EvalType() -> crate::Result<i32> {
    resolve("EvalType")
}
pub fn BindsTo() -> crate::Result<i32> {
    resolve("BindsTo")
}
pub fn Signature() -> crate::Result<i32> {
    resolve("Signature")
}
pub fn Local() -> crate::Result<i32> {
    resolve("Local")
}
pub fn InheritsFrom() -> crate::Result<i32> {
    resolve("InheritsFrom")
}
pub fn Tag() -> crate::Result<i32> {
    resolve("Tag")
}
pub fn Location() -> crate::Result<i32> {
    resolve("Location")
}
pub fn DispatchType() -> crate::Result<i32> {
    resolve("DispatchType")
}
pub fn ModifierType() -> crate::Result<i32> {
    resolve("ModifierType")
}
pub fn Identifier() -> crate::Result<i32> {
    resolve("Identifier")
}
pub fn ControlStructureType() -> crate::Result<i32> {
    resolve("ControlStructureType")
}
pub fn Contains() -> crate::Result<i32> {
    resolve("Contains")
}
pub fn Return() -> crate::Result<i32> {
    resolve("Return")
}
pub fn Block() -> crate::Result<i32> {
    resolve("Block")
}
pub fn MethodParameterOut() -> crate::Result<i32> {
    resolve("MethodParameterOut")
}
pub fn MethodParameterIn() -> crate::Result<i32> {
    resolve("MethodParameterIn")
}
pub fn Dependency() -> crate::Result<i32> {
    resolve("Dependency")
}
pub fn File() -> crate::Result<i32> {
    resolve("File")
}
pub fn MetaData() -> crate::Result<i32> {
    resolve("MetaData")
}
pub fn ArgumentIndex() -> crate::Result<i32> {
    resolve("ArgumentIndex")
}
pub fn Capture() -> crate::Result<i32> {
    resolve("Capture")
}
pub fn Namespace() -> crate::Result<i32> {
    resolve("Namespace")
}
pub fn CapturedBy() -> crate::Result<i32> {
    resolve("CapturedBy")
}
pub fn NamespaceBlock() -> crate::Result<i32> {
    resolve("NamespaceBlock")
}
pub fn Unknown() -> crate::Result<i32> {
    resolve("Unknown")
}
pub fn Type() -> crate::Result<i32> {
    resolve("Type")
}
pub fn TypeDecl() -> crate::Result<i32> {
    resolve("TypeDecl")
}
pub fn TypeParameter() -> crate::Result<i32> {
    resolve("TypeParameter")
}
pub fn TypeArgument() -> crate::Result<i32> {
    resolve("TypeArgument")
}
pub fn AnnotationLiteral() -> crate::Result<i32> {
    resolve("AnnotationLiteral")
}
pub fn ConfigFile() -> crate::Result<i32> {
    resolve("ConfigFile")
}
pub fn ClosureBindingId() -> crate::Result<i32> {
    resolve("ClosureBindingId")
}
pub fn TypeFullName() -> crate::Result<i32> {
    resolve("TypeFullName")
}
pub fn TypeDeclFullName() -> crate::Result<i32> {
    resolve("TypeDeclFullName")
}
pub fn InheritsFromTypeFullName() -> crate::Result<i32> {
    resolve("InheritsFromTypeFullName")
}
pub fn MethodFullName() -> crate::Result<i32> {
    resolve("MethodFullName")
}
pub fn Receiver() -> crate::Result<i32> {
    resolve("Receiver")
}
pub fn Condition() -> crate::Result<i32> {
    resolve("Condition")
}
pub fn AstParentType() -> crate::Result<i32> {
    resolve("AstParentType")
}
pub fn AstParentFullName() -> crate::Result<i32> {
    resolve("AstParentFullName")
}
pub fn DependencyGroupId() -> crate::Result<i32> {
    resolve("DependencyGroupId")
}
pub fn StaticReceiver() -> crate::Result<i32> {
    resolve("StaticReceiver")
}
pub fn TrueBody() -> crate::Result<i32> {
    resolve("TrueBody")
}
pub fn FalseBody() -> crate::Result<i32> {
    resolve("FalseBody")
}
pub fn DoBody() -> crate::Result<i32> {
    resolve("DoBody")
}
pub fn ForInit() -> crate::Result<i32> {
    resolve("ForInit")
}
pub fn ForUpdate() -> crate::Result<i32> {
    resolve("ForUpdate")
}
pub fn ForBody() -> crate::Result<i32> {
    resolve("ForBody")
}
pub fn TryBody() -> crate::Result<i32> {
    resolve("TryBody")
}
pub fn CatchBody() -> crate::Result<i32> {
    resolve("CatchBody")
}
pub fn FinallyBody() -> crate::Result<i32> {
    resolve("FinallyBody")
}
pub fn JumpArgument() -> crate::Result<i32> {
    resolve("JumpArgument")
}
pub fn Symbol() -> crate::Result<i32> {
    resolve("Symbol")
}
pub fn MethodShortName() -> crate::Result<i32> {
    resolve("MethodShortName")
}
pub fn PackageName() -> crate::Result<i32> {
    resolve("PackageName")
}
pub fn ClassName() -> crate::Result<i32> {
    resolve("ClassName")
}
pub fn NodeLabel() -> crate::Result<i32> {
    resolve("NodeLabel")
}
pub fn FileName() -> crate::Result<i32> {
    resolve("FileName")
}
pub fn Overlays() -> crate::Result<i32> {
    resolve("Overlays")
}
pub fn Hash() -> crate::Result<i32> {
    resolve("Hash")
}
pub fn ArgumentLabel() -> crate::Result<i32> {
    resolve("ArgumentLabel")
}
pub fn ArgumentName() -> crate::Result<i32> {
    resolve("ArgumentName")
}
pub fn Key() -> crate::Result<i32> {
    resolve("Key")
}
pub fn ClassShortName() -> crate::Result<i32> {
    resolve("ClassShortName")
}
pub fn ReachingDef() -> crate::Result<i32> {
    resolve("ReachingDef")
}
pub fn AliasOf() -> crate::Result<i32> {
    resolve("AliasOf")
}
pub fn Binding() -> crate::Result<i32> {
    resolve("Binding")
}
pub fn Binds() -> crate::Result<i32> {
    resolve("Binds")
}
pub fn Argument() -> crate::Result<i32> {
    resolve("Argument")
}
pub fn SourceFile() -> crate::Result<i32> {
    resolve("SourceFile")
}
pub fn AliasTypeFullName() -> crate::Result<i32> {
    resolve("AliasTypeFullName")
}
pub fn Dominate() -> crate::Result<i32> {
    resolve("Dominate")
}
pub fn PostDominate() -> crate::Result<i32> {
    resolve("PostDominate")
}
pub fn Cdg() -> crate::Result<i32> {
    resolve("Cdg")
}
pub fn TagNodePair() -> crate::Result<i32> {
    resolve("TagNodePair")
}
pub fn Finding() -> crate::Result<i32> {
    resolve("Finding")
}
pub fn KeyValuePair() -> crate::Result<i32> {
    resolve("KeyValuePair")
}
pub fn IsVariadic() -> crate::Result<i32> {
    resolve("IsVariadic")
}
pub fn Modifier() -> crate::Result<i32> {
    resolve("Modifier")
}
pub fn MethodRef() -> crate::Result<i32> {
    resolve("MethodRef")
}
pub fn ClosureBinding() -> crate::Result<i32> {
    resolve("ClosureBinding")
}
pub fn TypeRef() -> crate::Result<i32> {
    resolve("TypeRef")
}
pub fn ControlStructure() -> crate::Result<i32> {
    resolve("ControlStructure")
}
pub fn JumpTarget() -> crate::Result<i32> {
    resolve("JumpTarget")
}
pub fn JumpLabel() -> crate::Result<i32> {
    resolve("JumpLabel")
}
pub fn TemplateDom() -> crate::Result<i32> {
    resolve("TemplateDom")
}
pub fn Comment() -> crate::Result<i32> {
    resolve("Comment")
}
pub fn Root() -> crate::Result<i32> {
    resolve("Root")
}
pub fn DynamicTypeHintFullName() -> crate::Result<i32> {
    resolve("DynamicTypeHintFullName")
}
pub fn PossibleTypes() -> crate::Result<i32> {
    resolve("PossibleTypes")
}
pub fn Index() -> crate::Result<i32> {
    resolve("Index")
}
pub fn GenericSignature() -> crate::Result<i32> {
    resolve("GenericSignature")
}
pub fn Offset() -> crate::Result<i32> {
    resolve("Offset")
}
pub fn OffsetEnd() -> crate::Result<i32> {
    resolve("OffsetEnd")
}
pub fn Imports() -> crate::Result<i32> {
    resolve("Imports")
}
pub fn IsCallForImport() -> crate::Result<i32> {
    resolve("IsCallForImport")
}
pub fn FieldIdentifier() -> crate::Result<i32> {
    resolve("FieldIdentifier")
}
pub fn CanonicalName() -> crate::Result<i32> {
    resolve("CanonicalName")
}
pub fn ContainedRef() -> crate::Result<i32> {
    resolve("ContainedRef")
}
pub fn EvidenceDescription() -> crate::Result<i32> {
    resolve("EvidenceDescription")
}
