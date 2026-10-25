# FileSystem — JSON Schema dialect/vocabulary + Rust semantics

This is a declarative port of `FileSystem.scala` using JSON Schema Draft 2020-12 plus a FileSystem-specific CPG vocabulary implemented with `jsonschema` 0.60.1.

## Design

The JSON Schema document is the schema representation. There is no parallel Rust schema AST/IR. Rust only implements semantics that are awkward or impossible to express with ordinary JSON Schema alone:

- uniqueness of local property/node/edge names and per-category `protoId`s;
- FlatGraph `ValueType`/cardinality/default compatibility;
- imported-property resolution for `NAME`, `HASH`, and `CONTENT`;
- node property and primary-key resolution;
- validation of any explicit out-edge declarations without inventing endpoint triples;
- the `SOURCE_FILE` policy: automatically derived from `FILENAME`, never frontend-created, and targeting `FILE`;
- automatic `FILE` materialization from `FILENAME` when missing;
- the `OFFSET` / `OFFSET_END` relationship to `CONTENT`, UTF-16 offset encoding, and exclusive end semantics.

The semantic validator operates directly on `serde_json::Value`.

## Builder semantics preserved

The canonical model preserves the executable builder surface:

- `FILENAME`: `String`, mandatory, `PropertyDefaults.String`, proto ID 106;
- `LINE_NUMBER`: optional `Int`, proto ID 2;
- `COLUMN_NUMBER`: optional `Int`, proto ID 11;
- `LINE_NUMBER_END`: optional `Int`, proto ID 12;
- `COLUMN_NUMBER_END`: optional `Int`, proto ID 16;
- `OFFSET`: optional `Int`, proto ID 3812;
- `OFFSET_END`: optional `Int`, proto ID 3813;
- `FILE`: proto ID 38, properties `NAME`, `HASH`, `CONTENT`, primary key `NAME`;
- `SOURCE_FILE`: proto ID 157.

`PropertyDefaults.String` is resolved to the project-wide canonical value `<empty>`, matching the existing CpgSchema JSON binding.

`providedByFrontend` is `false` because `FileSystem` does not override `SchemaBase.providedByFrontend`, whose default is `false`.

## Prose semantics encoded

The Scala comments explicitly state that `SOURCE_FILE` edges MUST NOT be created by frontends and are automatically created from `FILENAME`; this is represented by `cpg:derivedEdges`.

The comments also state that missing `FILE` nodes are created from `FILENAME`; this is represented by `cpg:autoNodeMaterialization`.

The offset comments explicitly specify UTF-16 measurement and an exclusive `OFFSET_END`; this is represented by `cpg:offsetSemantics`.

## Deliberately not inferred

Two stronger claims are intentionally **not** invented:

1. The source does not explicitly state the lookup predicate used to choose a target `FILE` node (for example `FILENAME == FILE.NAME`), so the vocabulary records the target node type without asserting that unstated equality rule.
2. The `FILE` prose says file nodes are AST roots whose order is 0, but this builder does not import the AST layer, extend an AST base type, or add an `ORDER` property. The canonical schema records that statement in the node description rather than turning it into an unsupported structural constraint.

## Files

- `schemas/cpg-filesystem.vocabulary.meta.schema.json` — custom vocabulary syntax.
- `schemas/cpg-filesystem-dialect.meta.schema.json` — Draft 2020-12 dialect requiring the FileSystem vocabulary.
- `schemas/cpg-filesystem-layer.meta.schema.json` — FileSystem-specific meta-schema plus semantic assertion keyword.
- `schemas/filesystem.schema.json` — canonical declarative port.
- `src/semantic.rs` — cross-reference and static-semantic validation with no extra IR.
- `src/vocabulary.rs` — `jsonschema::Keyword` implementation.
- `src/lib.rs` — registry/dialect integration and public validation API.
- `build.rs` — build-time validation of the canonical schema.

## Run

```bash
cargo test --manifest-path src/FileSystem/Cargo.toml
```
