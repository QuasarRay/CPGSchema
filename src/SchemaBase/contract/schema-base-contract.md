# SchemaBase semantic contract

This document is the language- and serialization-neutral contract for a CPG
schema module. Neither the Rust API nor the JSON Schema dialect is normative
over the other; both are bindings of this contract.

## Semantic fields

| Semantic name | Domain | Required | Default | Meaning |
|---|---|---:|---|---|
| `doc_index` | signed 32-bit integer | yes | none | Documentation ordering key. Smaller values appear first. |
| `description` | Unicode string | yes | none | Human-readable description of the schema module. |
| `provided_by_frontend` | boolean | no | `false` | Whether the module is supplied by a language frontend. |

## Invariants

1. `doc_index` is in `[-2^31, 2^31 - 1]`.
2. `description` is present. The empty string is valid.
3. If `provided_by_frontend` is omitted by a binding, its semantic value is
   `false`.
4. A binding may add transport-, language-, or implementation-specific
   metadata, but it must not change the meaning of these three fields.
5. Consumers should depend on this semantic contract rather than on JSON field
   names or Rust identifier spelling.

## Bindings

### Rust

The Rust binding exposes:

```rust
pub trait SchemaBase {
    fn doc_index(&self) -> i32;
    fn description(&self) -> &str;
    fn provided_by_frontend(&self) -> bool { false }
}
```

The Rust binding uses snake_case names and does not require JSON support.

### JSON Schema dialect

The JSON binding maps the semantic fields as follows:

| Semantic field | JSON Schema representation |
|---|---|
| `doc_index` | `cpg:docIndex` |
| `description` | standard JSON Schema `description` |
| `provided_by_frontend` | `cpg:providedByFrontend` |

The JSON dialect additionally requires JSON-specific module identity fields
such as `$schema`, `$id`, and `title`. Those are transport/dialect requirements,
not members of the language-neutral SchemaBase contract.

`cpg:providedByFrontend` may be omitted. Its semantic default is `false`.
The JSON Schema `default` keyword is an annotation and does not mutate the
schema document; adapters must apply the semantic default when reading it.

## Compatibility rule

A new binding is compatible when it can faithfully expose all three semantic
fields and the default rule above. A binding need not use JSON Schema, Rust, or
the names used by either existing binding.
