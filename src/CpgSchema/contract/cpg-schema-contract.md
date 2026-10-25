# CpgSchema common contract

The stable project interface is semantic, not JSON-specific and not tied to
the concrete native implementation.

Rust expresses that interface through three traits:

- `SchemaModuleContract`
- `PropertyDefaultsContract`
- `CpgSchemaContract`

A non-Rust implementation can implement the same semantics independently.

## Builder identity

The canonical CPG schema exposes:

- domain short name: `Cpg`
- base package: `io.shiftleft.codepropertygraph.generated`
- additional traversal packages: `io.shiftleft`

## Module composition

Modules are ordered. Every dependency must refer to a module that appeared
earlier in that order. This is the semantic equivalent of constructing the
Scala modules sequentially with one shared builder.

## Property defaults

The common contract exposes the nine values from
`CpgSchema.PropertyDefaults`.

## Binding rules

### Native Rust

`src/native.rs` is a static, zero-JSON implementation.

### JSON Schema

`schemas/cpg.schema.json` is a JSON binding of the same contract.

The custom dialect and vocabulary define JSON spelling and schema-document
requirements. They do not redefine the project semantics.

### Interoperability

`JsonCpgSchema` parses the JSON binding and implements the same Rust traits as
`NativeCpgSchema`. Code consuming `CpgSchemaContract` therefore does not care
which binding supplied the schema.
