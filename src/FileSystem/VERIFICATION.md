# Verification

## Performed in this bundle

- All JSON files are parsed as JSON.
- The canonical FileSystem schema is mechanically checked for the exact property names, proto IDs, value types, cardinalities, imported FILE properties, FILE primary key, SOURCE_FILE proto ID, empty builder out-edge list, derived-edge policy, node materialization rule, and offset semantics.
- Rust code is structured against the same `jsonschema` 0.60.1 APIs already used by the repository's `src/Type` implementation: `Registry`, `meta::options`, `with_vocabulary`, `with_keyword`, and `Keyword`.

## Environment limitation

The execution container used to prepare this bundle does not contain `rustc`, `cargo`, or `rustfmt`, so an actual Rust compile/test run could not be performed here.

Run locally:

```bash
cargo test --manifest-path src/FileSystem/Cargo.toml
```

Then, if desired:

```bash
cargo fmt --manifest-path src/FileSystem/Cargo.toml -- --check
cargo clippy --manifest-path src/FileSystem/Cargo.toml --all-targets -- -D warnings
```
