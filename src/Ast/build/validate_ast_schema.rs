//! Include this file from the root crate's `build.rs`:
//!
//! ```rust
//! #[path = "build/validate_ast_schema.rs"]
//! mod validate_ast_schema;
//!
//! fn main() {
//!     validate_ast_schema::validate();
//! }
//! ```
//!
//! Required build dependencies:
//!
//! ```toml
//! [build-dependencies]
//! jsonschema = { version = "=0.60.1", default-features = false }
//! serde_json = "1.0"
//! ```

use serde_json::Value;
use std::{collections::BTreeSet, fs};

fn read(path: &str) -> Value {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    serde_json::from_str(&source)
        .unwrap_or_else(|error| panic!("invalid JSON in {path}: {error}"))
}

fn strings<'a>(value: &'a Value, key: &str) -> Vec<&'a str> {
    value
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("{key} must contain only strings"))
        })
        .collect()
}

fn ref_module(reference: &str) -> &str {
    reference
        .split("::")
        .next()
        .expect("reference must contain a module")
}

fn check_ref(reference: &str, known: &BTreeSet<String>, context: &str) {
    if !known.contains(reference) {
        panic!("{context} refers to `{reference}` before it is declared/imported");
    }
}

pub fn validate() {
    for path in [
        "schemas/cpg-graph.vocabulary.meta.schema.json",
        "schemas/ast-dialect.meta.schema.json",
        "schemas/ast.schema.json",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let meta = read("schemas/cpg-graph.vocabulary.meta.schema.json");
    let ast = read("schemas/ast.schema.json");

    jsonschema::meta::validate(&meta)
        .expect("CPG graph-schema vocabulary meta-schema must be valid JSON Schema");

    let validator = jsonschema::validator_for(&meta)
        .expect("CPG graph-schema vocabulary meta-schema must compile");
    validator
        .validate(&ast)
        .expect("ast.schema.json must conform to the graph-schema vocabulary meta-schema");

    let imports: BTreeSet<_> = strings(&ast, "cpg:imports").into_iter().collect();

    let externals = strings(&ast, "cpg:externals");
    let mut known = BTreeSet::<String>::new();

    for external in externals {
        let module = ref_module(external);
        if !imports.contains(module) {
            panic!(
                "external reference `{external}` uses module `{module}`, \
                 which is absent from cpg:imports"
            );
        }
        if !known.insert(external.to_owned()) {
            panic!("duplicate external reference `{external}`");
        }
    }

    let module = ast["cpg:moduleName"]
        .as_str()
        .expect("cpg:moduleName must be a string");

    let operations = ast["cpg:operations"]
        .as_array()
        .expect("cpg:operations must be an array");

    for (index, operation) in operations.iter().enumerate() {
        let kind = operation["kind"]
            .as_str()
            .unwrap_or_else(|| panic!("operation {index} has no kind"));

        match kind {
            "property" | "nodeBase" | "node" | "edge" => {
                for key in ["cpg:propertyRefs", "cpg:extends"] {
                    for reference in strings(operation, key) {
                        check_ref(reference, &known, &format!("operation {index}"));
                    }
                }

                if let Some(reference) = operation
                    .get("cpg:primaryKeyRef")
                    .and_then(Value::as_str)
                {
                    check_ref(reference, &known, &format!("operation {index}"));
                }

                let name = operation["name"]
                    .as_str()
                    .unwrap_or_else(|| panic!("operation {index} has no declaration name"));

                let ref_kind = match kind {
                    "property" => "property",
                    "nodeBase" => "nodeBase",
                    "node" => "node",
                    "edge" => "edge",
                    _ => unreachable!(),
                };

                let reference = format!("{module}::{ref_kind}::{name}");
                if !known.insert(reference.clone()) {
                    panic!("duplicate declaration `{reference}`");
                }
            }

            "constants" => {}

            "extend" => {
                let target = operation["target"].as_str().unwrap();
                check_ref(target, &known, &format!("operation {index}"));
                for reference in strings(operation, "cpg:extends") {
                    check_ref(reference, &known, &format!("operation {index}"));
                }
            }

            "outEdge" => {
                for key in ["source", "edge", "target"] {
                    let reference = operation[key].as_str().unwrap();
                    check_ref(reference, &known, &format!("operation {index}"));
                }
            }

            other => panic!("unknown operation kind `{other}` at index {index}"),
        }
    }
}
