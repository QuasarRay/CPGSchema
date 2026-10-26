# Integrated native module

Implementation milestone: **UNVERIFIED**. The production native implementation is now wired through this directory's `mod.rs` into the root `CPGSchema` package. Its independent JSON counterpart lives in `JsonSchema/src/Type/`. See the root `README.md` and `GRAPH_FORMAT.md` for current APIs and compilation usage.

The remaining notes describe the earlier prototype files retained in this directory; their architecture and verification statements do not describe the integrated packages. Existing reference and verification material is preserved.

---

# Type — JSON Schema dialect/vocabulary + Rust semantics

This is a direct declarative port of `Type.scala` using JSON Schema Draft 2020-12 plus a CPG-specific vocabulary implemented by `jsonschema` 0.60.1.

## Design

The JSON Schema document is the schema representation. There is no second Rust schema AST/IR. Rust only implements the static semantics that ordinary JSON Schema cannot express cleanly:

- uniqueness of property/node/edge names and per-category `protoId`s;
- property default compatibility with FlatGraph `ValueType` and cardinality;
- resolution of imported/local properties, node bases, node types, and edge types;
- inheritance and primary-key resolution;
- legal out-edge endpoint triples;
- consistency of loader-derived `ALIAS_OF` and `INHERITS_FROM` edges, without inventing alias target cardinality;
- cross-node property references such as `TYPE_DECL_FULL_NAME -> TYPE_DECL.FULL_NAME`.

The semantic validator works directly on `serde_json::Value`.

## Files

- `schemas/cpg-type.vocabulary.meta.schema.json` — vocabulary keyword syntax.
- `schemas/cpg-type-dialect.meta.schema.json` — Draft 2020-12 dialect requiring the Type vocabulary.
- `schemas/cpg-type-layer.meta.schema.json` — Type-layer-specific syntax plus the semantic assertion keyword.
- `schemas/type.schema.json` — canonical declarative port of `Type.scala`.
- `src/semantic.rs` — cross-reference/static-semantic checks, with no schema IR.
- `src/vocabulary.rs` — `jsonschema::Keyword` implementation.
- `build.rs` — registers the custom dialect, validates `$schema` through `jsonschema::meta`, and fails the build if the canonical Type schema violates syntax or semantics.

## Source conflict intentionally preserved

`TYPE_ARGUMENT`'s prose says the node MUST store `CODE`, but the Scala builder does not call `.addProperties(code)` for it. The canonical JSON keeps the executable builder behavior (`properties: []`) and records the discrepancy in the description instead of silently changing the source semantics.

## Run

```bash
cargo test --manifest-path src/Type/Cargo.toml
```
