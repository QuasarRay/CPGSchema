//! Native port of Operators.scala. Source revision: crate::SOURCE_REVISION.
use crate::model::{Constant, ConstantCategoryId, ModuleInfo, Result, SchemaBuilder};
use crate::value::ValueType;
pub const MODULE_NAME: &str = "Operators";
pub const DOC_INDEX: i32 = 17;
pub const DESCRIPTION: &str = r#"

"#;
pub const PROVIDED_BY_FRONTEND: bool = false;
#[derive(Clone, Debug)]
pub struct Schema {
    pub operators: ConstantCategoryId,
}
impl Schema {
    pub fn new(builder: &mut SchemaBuilder) -> Result<Self> {
        builder.add_module(ModuleInfo {
            name: MODULE_NAME.into(),
            doc_index: DOC_INDEX,
            description: DESCRIPTION.into(),
            provided_by_frontend: PROVIDED_BY_FRONTEND,
            dependencies: vec![],
        })?;

        let operators = builder.add_constants("Operators", vec![
    Constant::new("addition", "<operator>.addition", ValueType::String, "", MODULE_NAME),
    Constant::new("subtraction", "<operator>.subtraction", ValueType::String, "", MODULE_NAME),
    Constant::new("multiplication", "<operator>.multiplication", ValueType::String, "", MODULE_NAME),
    Constant::new("division", "<operator>.division", ValueType::String, "", MODULE_NAME),
    Constant::new("exponentiation", "<operator>.exponentiation", ValueType::String, "", MODULE_NAME),
    Constant::new("modulo", "<operator>.modulo", ValueType::String, "", MODULE_NAME),
    Constant::new("shiftLeft", "<operator>.shiftLeft", ValueType::String, "", MODULE_NAME),
    Constant::new("logicalShiftRight", "<operator>.logicalShiftRight", ValueType::String, "Shift right padding with zeros", MODULE_NAME),
    Constant::new("arithmeticShiftRight", "<operator>.arithmeticShiftRight", ValueType::String, "Shift right preserving the sign", MODULE_NAME),
    Constant::new("not", "<operator>.not", ValueType::String, "", MODULE_NAME),
    Constant::new("and", "<operator>.and", ValueType::String, "", MODULE_NAME),
    Constant::new("or", "<operator>.or", ValueType::String, "", MODULE_NAME),
    Constant::new("xor", "<operator>.xor", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentPlus", "<operator>.assignmentPlus", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentMinus", "<operator>.assignmentMinus", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentMultiplication", "<operator>.assignmentMultiplication", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentDivision", "<operator>.assignmentDivision", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentExponentiation", "<operators>.assignmentExponentiation", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentModulo", "<operators>.assignmentModulo", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentShiftLeft", "<operators>.assignmentShiftLeft", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentLogicalShiftRight", "<operators>.assignmentLogicalShiftRight", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentArithmeticShiftRight", "<operators>.assignmentArithmeticShiftRight", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentAnd", "<operators>.assignmentAnd", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentOr", "<operators>.assignmentOr", ValueType::String, "", MODULE_NAME),
    Constant::new("assignmentXor", "<operators>.assignmentXor", ValueType::String, "", MODULE_NAME),
    Constant::new("arrayInitializer", "<operator>.arrayInitializer", ValueType::String, "", MODULE_NAME),
    Constant::new("assignment", "<operator>.assignment", ValueType::String, "", MODULE_NAME),
    Constant::new("minus", "<operator>.minus", ValueType::String, "E.g. `a = -b`", MODULE_NAME),
    Constant::new("plus", "<operator>.plus", ValueType::String, "E.g. `a = +b`", MODULE_NAME),
    Constant::new("preIncrement", "<operator>.preIncrement", ValueType::String, "", MODULE_NAME),
    Constant::new("preDecrement", "<operator>.preDecrement", ValueType::String, "", MODULE_NAME),
    Constant::new("postIncrement", "<operator>.postIncrement", ValueType::String, "", MODULE_NAME),
    Constant::new("postDecrement", "<operator>.postDecrement", ValueType::String, "", MODULE_NAME),
    Constant::new("logicalNot", "<operator>.logicalNot", ValueType::String, "", MODULE_NAME),
    Constant::new("logicalOr", "<operator>.logicalOr", ValueType::String, "", MODULE_NAME),
    Constant::new("logicalAnd", "<operator>.logicalAnd", ValueType::String, "", MODULE_NAME),
    Constant::new("equals", "<operator>.equals", ValueType::String, "", MODULE_NAME),
    Constant::new("notEquals", "<operator>.notEquals", ValueType::String, "", MODULE_NAME),
    Constant::new("greaterThan", "<operator>.greaterThan", ValueType::String, "", MODULE_NAME),
    Constant::new("lessThan", "<operator>.lessThan", ValueType::String, "", MODULE_NAME),
    Constant::new("greaterEqualsThan", "<operator>.greaterEqualsThan", ValueType::String, "", MODULE_NAME),
    Constant::new("lessEqualsThan", "<operator>.lessEqualsThan", ValueType::String, "", MODULE_NAME),
    Constant::new("instanceOf", "<operator>.instanceOf", ValueType::String, "", MODULE_NAME),
    Constant::new("memberAccess", "<operator>.memberAccess", ValueType::String, "Deprecated. Formerly in C: `a.b` but not! in Java", MODULE_NAME),
    Constant::new("indirectMemberAccess", "<operator>.indirectMemberAccess", ValueType::String, "Deprecated. Formerly in C: `a->b` and `a.b` in Java", MODULE_NAME),
    Constant::new("computedMemberAccess", "<operator>.computedMemberAccess", ValueType::String, "Deprecated. Formerly in C: `a[b]` but not! in Java", MODULE_NAME),
    Constant::new("indirectComputedMemberAccess", "<operator>.indirectComputedMemberAccess", ValueType::String, "Deprecated. Formerly in C++: `a->*b` and a[b] in Java", MODULE_NAME),
    Constant::new("indirection", "<operator>.indirection", ValueType::String, "Accesses through a pointer (load/store), i.e. dereferences it. Typically the star-operator in C/C++. Pairs of addressOf and indirection operators cancel each other. Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("delete", "<operator>.delete", ValueType::String, "Deletes a property from a namespace. E.g. `a=3; delete a; a == undefined;", MODULE_NAME),
    Constant::new("conditional", "<operator>.conditional", ValueType::String, "E.g. `a ? consequent : alternate`. In future probably also used for if statements", MODULE_NAME),
    Constant::new("elvis", "<operator>.elvis", ValueType::String, "E.g. `x = f() ?: g()`. A binary operator that returns its first operand if that operand evaluates to a true value, and otherwise evaluates and returns its second operand.", MODULE_NAME),
    Constant::new("cast", "<operator>.cast", ValueType::String, "Type casts of any sort", MODULE_NAME),
    Constant::new("compare", "<operator>.compare", ValueType::String, "Comparison between two arguments with the results: 0 == equal, negative == left < right, positive == left > right", MODULE_NAME),
    Constant::new("addressOf", "<operator>.addressOf", ValueType::String, "Returns the address of a given object. Pairs of addressOf and indirection operators cancel each other. Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("sizeOf", "<operator>.sizeOf", ValueType::String, "Returns the size of a given object", MODULE_NAME),
    Constant::new("fieldAccess", "<operator>.fieldAccess", ValueType::String, "Returns or sets a field/member of a struct/class. Typically, the dot-operator. In terms of CPG, the first argument is the struct/class and the second argument is either a FIELD_IDENTIFIER node, a LITERAL node or an arbitrary expression. If it is a FIELD_IDENTIFIER, then the string contained in the CANONICAL_NAME field of this FIELD_IDENTIFIER node selects which field/member is accessed. If it is a LITERAL node, then its CODE selects which field/member is selected. In any other case the access is considered unpredictable by the backend, and we overtaint. indexAccess and fieldAccess live in the same space, such that e.g. obj.field and obj[\"field\"] refer to the same target (as is correct in e.g. javascript). Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("indirectFieldAccess", "<operator>.indirectFieldAccess", ValueType::String, "Accesses (loads/stores) to a field/member through a pointer to a struct/class. Typically, C/C++ arrow-operator. The field selection works the same way as for fieldAccess. For the sake of data-flow tracking, this is the same as first dereferencing the pointer and then accessing the field. Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("indexAccess", "<operator>.indexAccess", ValueType::String, "Accesses a container (e.g. array or associative array / dict / map) at an index. The index selection works the same way as for fieldAccess. Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("indirectIndexAccess", "<operator>.indirectIndexAccess", ValueType::String, "Accesses an element of an Array through a pointer. The index selection works similar as for fieldAccess: However, the index must be an integer. If the second argument is a FIELD_ACCESS resp. LITERAL then its CANONICAL_NAME resp. CODE field is parsed as an integer; on parsing failure, the access is considered unpredictable and we overtaint. This is equivalent to a pointerShift followed by an indirection. This operator is currently only used by C-style languages. This behavior allows the backend to infer that ptr[0] and *ptr refer to the same object. Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("pointerShift", "<operator>.pointerShift", ValueType::String, "Shifts a pointer. In terms of CPG, the first argument is the pointer and the second argument is the index. The index selection works the same way as for indirectIndexAccess. This operator is currently only used directly by the LLVM language, but it is also used internally for C. For example, pointerShift(ptr, 7) is equivalent to &(ptr[7]). Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("getElementPtr", "<operator>.getElementPtr", ValueType::String, "Derives a pointer to a field of a struct from a pointer to the entire struct. This corresponds to the C idiom &(ptr->field), which does not access memory. The field selection works the same way as for fieldAccess. This operator is currently only used directly by the LLVM language. Handling of this operator is special-cased in the back-end", MODULE_NAME),
    Constant::new("formatString", "<operator>.formatString", ValueType::String, "Defines a string literal which can replace containing expressions with their values.", MODULE_NAME),
    Constant::new("formattedValue", "<operator>.formattedValue", ValueType::String, "Replaces an expression with its string value.", MODULE_NAME),
    Constant::new("range", "<operator>.range", ValueType::String, "Defines a range of values, e.g. `for (i in 1..10) print(i)`.", MODULE_NAME),
    Constant::new("in", "<operator>.in", ValueType::String, "Checks the existence of a variable in a range or collection, e.g. `for (i in 1..10) print(i)`.", MODULE_NAME),
    Constant::new("notIn", "<operator>.notIn", ValueType::String, "Checks the non-existence of a variable in a range or collection, e.g. `print(5 not in [1, 2, 3, 4])`", MODULE_NAME),
    Constant::new("is", "<operator>.is", ValueType::String, "Checks whether a variable is of a given type", MODULE_NAME),
    Constant::new("isNot", "<operator>.isNot", ValueType::String, "Checks whether a variable is not of a given type", MODULE_NAME),
    Constant::new("notNullAssert", "<operator>.notNullAssert", ValueType::String, "Converts any value to a not-null type", MODULE_NAME),
    Constant::new("lengthOf", "<operator>.lengthOf", ValueType::String, "Returns the length of the given collection e.g. (new int[]{ 1, 2, 3 }).length in Java or len([1, 2, 3]) in Python", MODULE_NAME),
    Constant::new("safeNavigation", "<operator>.safeNavigation", ValueType::String, "Returns null if the first operator is null, otherwise performs a dereferencing operation", MODULE_NAME),
    Constant::new("tryCatch", "<operator>.tryCatch", ValueType::String, "Returns either the last expression in the `try`-block, or the last expression in one of the `catch`/`except`-blocks. The contents of the `finally`-block are ignored.", MODULE_NAME),
    Constant::new("alloc", "<operator>.alloc", ValueType::String, "Allocates memory for an object of a specific type", MODULE_NAME),
    Constant::new("importCall", "<operator>.import", ValueType::String, "Imports a package", MODULE_NAME)
])?;
        Ok(Self { operators })
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
