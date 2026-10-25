# CpgSchema — common traits + native Rust + compile-time JSON dialect

This bundle ports the supplied `CpgSchema.scala` and replaces the earlier
Rust-only builder representation with one common semantic interface and two
bindings.

## Architecture

```text
                 CpgSchema semantic contract
                    Rust traits / documented semantics
                              │
                  ┌───────────┴───────────┐
                  │                       │
                  ▼                       ▼
          Native Rust binding       JSON Schema binding
             `native.rs`              `cpg.schema.json`
                  │                       │
                  │              CPG dialect + vocabulary
                  │                       │
                  └───────────┬───────────┘
                              ▼
                      `CpgSchemaContract`
```

## Files

### Common interface

- `src/contracts.rs`
  - `SchemaModuleContract`
  - `PropertyDefaultsContract`
  - `CpgSchemaContract`
  - representation-independent `validate_contract`

- `contract/cpg-schema-contract.md`
  - format/language-neutral semantics

### Native Rust

- `src/native.rs`
  - static module graph
  - native property defaults
  - `NativeCpgSchema`
  - no JSON/serde dependency

### JSON dialect/vocabulary

- `schemas/cpg-schema.vocabulary.meta.schema.json`
  - syntax of `cpg:*` vocabulary keywords

- `schemas/cpg-dialect.meta.schema.json`
  - Draft 2020-12 + required CPG vocabulary

- `schemas/cpg-schema.meta.schema.json`
  - MetaSchema for a CpgSchema composition document

- `schemas/cpg.schema.json`
  - canonical declarative CpgSchema port

- `src/json_vocabulary.rs`
  - compile-time `jsonschema` keyword factories

- `src/json_binding.rs`
  - compile-time generated validators
  - `JsonCpgSchema`, implementing the common Rust traits

- `build.rs`
  - build-time MetaSchema validation
  - build-time ordered-dependency validation

## Compile-time behavior

`jsonschema`'s `#[validator]` macro compiles the canonical JSON schema during
Rust compilation. The custom dialect resource, required vocabulary URI, and
custom keyword factories are supplied to the macro.

`build.rs` additionally validates `cpg.schema.json` against
`cpg-schema.meta.schema.json` and checks the ordered dependency graph. Thus an
invalid static JSON CpgSchema fails `cargo build`.

There is no runtime JSON Schema registry and no runtime schema compilation in
the application.

## Feature isolation

Native-only:

```bash
cargo test
```

Native + JSON dialect:

```bash
cargo test --features json-dialect
```

Consumers that only need the common interface can depend exclusively on
`contracts.rs`.

## Source semantics preserved

The implementation preserves:

- the builder configuration;
- all 21 modules;
- original module construction order;
- every direct dependency;
- the nine `PropertyDefaults` values.
