# SchemaBase MetaSchema + dual binding

This package separates the common CPG schema-module contract from its language
and serialization bindings.

## Layers

```text
contract/schema-base-contract.md
            │
      language-neutral
            │
       ┌────┴────┐
       │         │
       ▼         ▼
 Rust binding   JSON Schema binding
 schema_base.rs schemas/*.json
       │         │
       └────┬────┘
            ▼
      optional bridge.rs
```

### 1. Language-neutral contract

`contract/schema-base-contract.md` defines only:

- documentation order (`i32` semantic range),
- description,
- frontend-provided flag with semantic default `false`.

It deliberately does not define Rust names or JSON field names.

### 2. Rust-native interface

`src/schema_base.rs` contains a zero-dependency native trait and borrowed value
view. It is usable with no JSON support enabled.

### 3. MetaSchema vocabulary

`schemas/schema-base.vocabulary.meta.schema.json` defines the JSON binding of
the language-neutral fields:

- standard `description`
- `cpg:docIndex`
- `cpg:providedByFrontend`

### 4. JSON dialect

`schemas/cpg-dialect.meta.schema.json` composes Draft 2020-12 with the
SchemaBase vocabulary and adds JSON-specific document requirements:

- `$schema`
- `$id`
- `title`

These are JSON binding requirements, not members of the neutral contract.

### 5. Compile-time jsonschema integration

With feature `json-dialect`, `src/json_dialect.rs` uses
`#[jsonschema::validator]` to compile:

- the SchemaBase vocabulary meta-schema,
- the CPG dialect meta-schema,
- a representative schema document using the custom dialect.

There is no runtime registry or runtime schema compilation.

### 6. Optional bridge

Feature `bridge` exposes `JsonSchemaBase<'a>`, which validates a JSON schema
document and adapts it to the native Rust `SchemaBase` trait. The semantic
default for omitted `cpg:providedByFrontend` is applied here.

## Cargo

Rust-only contract:

```bash
cargo test
```

JSON binding:

```bash
cargo test --features json-dialect
```

JSON ↔ Rust bridge:

```bash
cargo test --features bridge
```

## Integration

If this is copied into a larger project rather than used as its own crate,
preserve the separation:

- core modules depend on `schema_base`;
- JSON-specific modules depend on `json_dialect`;
- only code that needs translation between the two depends on `bridge`.
