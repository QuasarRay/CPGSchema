# CPGSchema

Implementation milestone: **UNVERIFIED**. Compilation is used during implementation; this milestone includes no tests, proofs, source-comparison runs, or correctness reports.

The root `CPGSchema` package implements the Code Property Graph schema in native Rust. The independent `cpg-schema-json` package in `JsonSchema/` implements its own JSON Schema dialect, schema interpretation, and graph validation. Neither production implementation depends on the other. Their initial reference is codepropertygraph commit `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` with Flatgraph `0.1.36`.

The source-compatible modules retain their Scala spelling. `CpgSchema::CpgSchema::new` composes all modules over a shared native `SchemaBuilder`; each module exposes its own `Schema` with typed handles for the definitions it owns. Later modules can modify earlier definitions. Builder storage retains declaration order and handle identity; resolved property and concrete-node accessors use the source's sorted presentation order. Original Scala references and earlier experimental bundles remain available alongside the integrated modules.

```rust
use CPGSchema::{CpgSchema, graph::{Graph, ValidationOptions, ConstructionStage}};
use CPGSchema::components::{NativeContext, CanProvideSchema, CanValidateGraph};

let schema = NativeContext.provide_schema()?;
let graph = Graph::default();
let options = ValidationOptions::all(&schema, ConstructionStage::Frontend);
let diagnostics = NativeContext.validate_graph(&schema, &graph, &options);
# Ok::<(), CPGSchema::Error>(())
```

`Schema::validate` and `graph::validate_graph` are production APIs. Schema construction enforces the source's per-namespace protobuf identifier uniqueness. Graph validation returns structured diagnostics without changing input. `graph::materialize_defaults` is an explicit copying operation; validation never fills missing values. Construction stage and active layers are explicit. Frontend output can omit loader-derived structures but must respect applicable structural limits. Materialized validation includes applicable derived relationships and source contracts. An active layer requires its declared dependencies.

Native values use signed integers, raw Float/Double bits, UTF-16 strings, and UTF-16 Char code units. Property cardinality lists and scalar List values are different types. Contained-node references retain their separate cardinality and null defaults. `CONTAINED_REF` remains protocol metadata and is not accepted as an ordinary graph property.

The optional `json-io` feature enables the native CPGSchemaGraph codec. It does not load JSON schema definitions or depend on the dialect package. See [GRAPH_FORMAT.md](GRAPH_FORMAT.md) for the wire format.

CGP contexts select schema composition/provision, schema validation, graph validation, value interpretation, and protobuf export providers. The native defaults are wired in `components::NativeContext`. `derive-deftly` generates value-type name projection from the actual enum. Schema facts remain in their owning modules.

`Protogen::schema` constructs a schema with the source's protobuf options; `Protogen::generate` returns declarations and `Protogen::run` writes them. The `Protogen` binary accepts an output directory. Its common protobuf layout follows Flatgraph 0.1.36, including overlays, DiffGraph messages, and reserved Edge fields. Scalar graph transport is separate from protobuf's narrower value representation.

Compilation while implementing:

```sh
cargo check --package CPGSchema --lib --bins --features json-io
cargo check --package cpg-schema-json --lib --bins
```

The dependency lockfile fixes the dependency resolution. CGP uses the requested immutable Git revision, derive-deftly is pinned to 1.14.0, and the dialect uses jsonschema 0.60.1. The local dependency checkouts are reference material and are not modified by these packages.

Native and JSON definitions and semantic implementations are maintained separately. A future synchronization is an explicit edit to each implementation. Builds do not synchronize, regenerate, or compare them.
