# Integrated native module

Implementation milestone: **UNVERIFIED**. The production native implementation is now wired through this directory's `mod.rs` into the root `CPGSchema` package. Its independent JSON counterpart lives in `JsonSchema/src/Ast/`. See the root `README.md` and `GRAPH_FORMAT.md` for current APIs and compilation usage.

The remaining notes describe the earlier prototype files retained in this directory; their architecture and verification statements do not describe the integrated packages. Existing reference and verification material is preserved.

---

# AST compile-time JSON Schema dialect

This bundle is intended to be merged into an existing Cargo crate. It is NOT a
nested standalone crate.

## Copy layout

Copy the files so the project root contains:

```text
Cargo.toml
build.rs
build/
└── validate_ast_schema.rs
schemas/
├── cpg-graph.vocabulary.meta.schema.json
├── ast-dialect.meta.schema.json
└── ast.schema.json
src/
├── ast_vocabulary.rs
├── ast_jsonschema.rs
└── ...
```

Do not place a nested `Cargo.toml` or nested `src/lib.rs` under `src/Ast/`.

## Design

`ast.schema.json` is an ordered schema-builder program, not a set of unordered
JSON object maps. `cpg:operations` preserves the original Scala execution order,
including declarations, `extendz` mutations, and every `addOutEdge` call.

The dialect has two levels:

1. `cpg-graph.vocabulary.meta.schema.json`
   - defines the syntax of graph-schema operations;
2. `ast-dialect.meta.schema.json`
   - composes Draft 2020-12 with that vocabulary and marks the vocabulary
     required.

`ast_vocabulary.rs` provides the custom keyword hooks required by
`jsonschema`.

`ast_jsonschema.rs` uses `#[jsonschema::validator]`, so the concrete AST schema
is compiled into Rust validation code during `cargo build`. There is no runtime
schema registry.

`build/validate_ast_schema.rs` performs the other compile-time obligation:
validating the schema document itself and checking ordered symbolic references.

## Semantics represented

The source is encoded as:

- 4 properties
- 3 node base types
- 14 concrete node types
- 2 constant categories / 28 constants
- 12 edge types
- 19 standalone inheritance mutations
- 180 outgoing edge declarations

The schema preserves explicit:

- ProtoIds
- mandatory defaults
- property references
- inheritance
- primary keys
- starter names
- edge source/target types
- `cardinalityIn` / `cardinalityOut`
- traversal step names and documentation
- imported schema references

## Root `lib.rs`

Add:

```rust
mod ast_vocabulary;
mod ast_jsonschema;
```

## Root `build.rs`

If there is no existing build script:

```rust
#[path = "build/validate_ast_schema.rs"]
mod validate_ast_schema;

fn main() {
    validate_ast_schema::validate();
}
```

If a root `build.rs` already exists, add the module declaration and call
`validate_ast_schema::validate()` from the existing `main`.

## Cargo dependencies

See `Cargo.toml.additions.toml`. Merge those dependency entries into the root
Cargo.toml rather than replacing unrelated project dependencies.
