# Independent CPG JSON Schema dialect

Implementation milestone: **UNVERIFIED**. This package owns its definitions, interpretation, value model, codec, semantic rules, and protobuf exporter. It has no dependency on the root native package. No tests, proofs, comparison runs, or correctness reports are part of this milestone.

The reference is codepropertygraph revision `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` and its Flatgraph 0.1.36 schema semantics. Source-compatible module names and nested `Schema` interfaces are preserved. Each module's independently maintained `schema.json` is the declaration program for that module. Operations remain ordered, including later mutations to imported nodes and bases.

`src/CpgSchema/schema.json` owns the canonical builder configuration, composition order, and property defaults. `src/ProtoIds/schema.json` owns the named protocol identifiers; source names are retained as resolver functions in `ProtoIds`. `src/Protogen/schema.json` owns protobuf options. Numeric protocol identifiers and explicit scalar defaults are also supported in schema documents. Named `$protoId` and `$cpgDefault` references resolve against this implementation's documents and context. Changes to these resources do not change the native package.

`interpreter::canonical_bundle` assembles this package's module documents into an ordinary schema bundle. A bundle contains `cpg:builder` and either inline `cpg:modules` or `cpg:composition` naming bundled resources. Inline modules allow callers to define their own complete schema bundle. `interpreter::interpret_bundle` validates syntax, interprets ordered operations, resolves references and inheritance, and applies schema invariants. Imports must resolve to earlier declared dependencies. A module cannot access another module's symbols without declaring that dependency.

Schema handles retain identity across module mutations. `Schema::properties`, `Schema::node_base_types`, `Schema::concrete_nodes`, and `Schema::edge_types` expose sorted schema views; internal vectors retain construction order. Adjacency access preserves incoming and outgoing cardinality and traversal metadata independently. The schema retains direct and inherited properties, contained nodes, primary keys, constants, protocol IDs, and documentation metadata.

The four validation surfaces are explicit:

- `dialect::validate_document_metaschema` validates a schema against its declared dialect. The shipped dialect and vocabulary documents can themselves be checked against Draft 2020-12 through this API.
- `dialect::validate_module_syntax` and `validate_bundle_syntax` check schema-document shape.
- `interpreter::interpret_bundle`, `Schema::validate`, and the `cpg:schema` assertion implement schema semantics.
- `graph::validate_graph` and the `cpg:graph` assertion implement graph-instance semantics. `validate_graph_syntax` checks only the transport shape.

The dialect URI is `https://quasarray.github.io/CPGSchema/dialect/1`, and the vocabulary URI is `https://quasarray.github.io/CPGSchema/vocabulary/1`. The resources are shipped locally. These identifiers do not require a hosted endpoint. The library does not enable network retrieval.

The vocabulary's definition keywords (`cpg:module`, `cpg:imports`, `cpg:operations`, builder configuration, defaults, protocol identifiers, and metadata) describe the schema program. They are interpreted by the ordered interpreter. They are not individually advertised as graph assertions. Compile a graph validator with `Dialect::compile_graph` to apply the complete graph semantics. Registering the vocabulary URI alone is insufficient; `Dialect::compile` installs the actual `cpg:schema` and `cpg:graph` keyword implementations.

`cpg:schema: true` asserts that an instance is a complete, semantically valid schema bundle; false explicitly disables that assertion. `cpg:graph` takes `schema`, `stage`, and `layers`. `schema` is the complete bundle; `stage` must be `Frontend` or `Materialized`; `layers` is an explicit array. The keyword checks graph format, values, identities, references, inheritance, endpoint legality, cardinalities, contained-node fields, and applicable stage-sensitive source contracts. It does not mutate instances.

```rust
use cpg_schema_json::{dialect::Dialect, interpreter, graph::ConstructionStage};
let bundle = interpreter::canonical_bundle()?;
let schema = interpreter::interpret_bundle(&bundle)?;
let layers: Vec<String> = schema.modules.iter().map(|m| m.name.clone()).collect();
let validator = Dialect.compile_graph(&bundle, ConstructionStage::Materialized, &layers)?;
# Ok::<(), cpg_schema_json::Error>(())
```

Retain the returned compiled validator for repeated validation. Module, bundle and graph syntax validators are cached. The returned `Validator` offers jsonschema's normal validation API; direct `graph::validate_graph` returns the package's structured diagnostics with entity IDs and paths.

CGP wiring is owned by `components::JsonContext`, independently selecting composition, provision, schema and graph validation, value interpretation, and protobuf export providers. The value-type enum owns a derive-deftly template for its name projection. Ordinary Rust implements the semantic rules.

The graph transport is documented in the repository's `GRAPH_FORMAT.md`. This package's codec preserves signed integer widths, Float/Double bits, UTF-16 characters and strings, missing properties, list order, and repeated edge occurrences. `graph::materialize_defaults` is explicit and returns a copy. `CONTAINED_REF` is serialization metadata, not an ordinary instance property. Mandatory contained-node null defaults are represented separately from ordinary property values.

`CpgSchema::instance` constructs a schema from this package's JSON definitions. Per-module `Schema::new` methods can compose modules into an existing builder; `Schema::from_builder` exposes the corresponding typed handles after composition. `Protogen::generate` and `Protogen::run` export protobuf declarations using this package's own schema and configuration. The `JsonSchemaProtogen` binary accepts an output directory.

Compile during implementation with `cargo check --package cpg-schema-json --lib --bins`. The existing project license and upstream attributions are documented in the repository's `LICENSE.MD` and `THIRD_PARTY.md`.

Synchronize the native and JSON implementations only through an explicit maintenance edit. No build script, source include, runtime import, shared semantic library, or regeneration hook links the two production implementations.
