use serde_json::Value;
use std::{collections::BTreeSet, fs};

fn read(path: &str) -> Value {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("invalid JSON in {path}: {error}"))
}

fn validate_ordered_module_graph(value: &Value) {
    let modules = value["cpg:modules"]
        .as_array()
        .expect("cpg:modules must be an array");

    let mut initialized = BTreeSet::<String>::new();

    for (index, module) in modules.iter().enumerate() {
        let name = module["name"]
            .as_str()
            .unwrap_or_else(|| panic!("module {index} has no string name"));

        if !initialized.insert(name.to_owned()) {
            panic!("duplicate CPG module `{name}`");
        }

        let deps = module["cpg:dependsOn"]
            .as_array()
            .unwrap_or_else(|| panic!("module `{name}` has no dependency array"));

        let mut local = BTreeSet::<String>::new();
        for dep in deps {
            let dep = dep
                .as_str()
                .unwrap_or_else(|| panic!("module `{name}` has non-string dependency"));

            if !local.insert(dep.to_owned()) {
                panic!("module `{name}` repeats dependency `{dep}`");
            }

            if !initialized.contains(dep) {
                panic!(
                    "module `{name}` depends on `{dep}`, but `{dep}` has not appeared earlier"
                );
            }
        }
    }
}

fn main() {
    for path in [
        "schemas/cpg-schema.vocabulary.meta.schema.json",
        "schemas/cpg-dialect.meta.schema.json",
        "schemas/cpg-schema.meta.schema.json",
        "schemas/cpg.schema.json",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let vocabulary_meta = read("schemas/cpg-schema.vocabulary.meta.schema.json");
    let cpg_meta = read("schemas/cpg-schema.meta.schema.json");
    let canonical = read("schemas/cpg.schema.json");

    // Validate the two standard Draft 2020-12 meta-schemas themselves.
    jsonschema::meta::validate(&vocabulary_meta)
        .expect("CPG vocabulary meta-schema must be a valid JSON Schema");
    jsonschema::meta::validate(&cpg_meta)
        .expect("CpgSchema meta-schema must be a valid JSON Schema");

    // Validate the canonical JSON binding against the CpgSchema MetaSchema.
    let validator = jsonschema::validator_for(&cpg_meta)
        .expect("CpgSchema meta-schema must compile");
    validator
        .validate(&canonical)
        .expect("canonical cpg.schema.json must conform to CpgSchema MetaSchema");

    // Cross-reference/order semantics are part of the common CpgSchema contract
    // and are not naturally expressible with ordinary JSON Schema keywords.
    validate_ordered_module_graph(&canonical);
}
