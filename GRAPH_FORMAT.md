# CPGSchemaGraph, version 1

Implementation milestone: **UNVERIFIED**.

A graph document is an object with exactly `format`, `formatVersion`, `schemaRevision`, `nodes`, and `edges`. `format` is `CPGSchemaGraph`; `formatVersion` is `1`; `schemaRevision` names the pinned Code Property Graph revision. Nodes and edges are arrays whose order and repeated edge occurrences are preserved. Object member order is not graph semantics.

A node contains `id`, `label`, `properties`, and optional `contained`. An edge contains `id`, `source`, `target`, `label`, and `properties`. Identifiers are nonempty strings, unique within their own node or edge namespace. Edge endpoints name existing node identifiers. Labels and property names retain their exact source spelling. Transport edge identifiers distinguish occurrences; the upstream protobuf Edge message does not have an edge key.

`properties` is an object. An ordinary scalar value is a tagged object. A list-valued property is an array of tagged scalar values, preserving order and duplicates. An absent property, a present empty list, and an explicit default are distinct. Null is not an ordinary scalar value.

| Type | Encoding |
| --- | --- |
| Boolean | `{"type":"Boolean","value":true}` |
| Byte, Short, Int, Long | `{"type":"Long","value":"-9223372036854775808"}`; canonical signed decimal within the selected signed width |
| Float | `{"type":"Float","bits":"80000000"}`; exactly eight lowercase hexadecimal digits |
| Double | `{"type":"Double","bits":"8000000000000000"}`; exactly sixteen lowercase hexadecimal digits |
| Char | `{"type":"Char","value":55296}`; one UTF-16 code unit, including isolated surrogates |
| String | `{"type":"String","value":"text"}` or `{"type":"String","utf16":[55296]}`; exactly one representation |
| NodeRef | `{"type":"NodeRef","value":"node-id"}` |
| List scalar | `{"type":"List","value":[{"type":"Int","value":"1"}]}`; distinct from property cardinality List |

Float and Double encodings preserve signed zero, infinity and NaN payloads. Unicode strings are not normalized. The UTF-16 alternative preserves strings that UTF-8 cannot represent. A writer can emit the Unicode form when the same code units form valid Unicode. JVM offsets address UTF-16 code units; slicing may retain an isolated surrogate.

`contained` maps declared local contained-node names to a node identifier, an array of identifiers, or an explicit null for a source-declared mandatory null default. Optional absence is represented by omitting the field. Contained nodes are separate graph nodes, and references must have compatible schema types.

Graph validation takes construction stage (`Frontend` or `Materialized`) and active layers as explicit API arguments; they are not inferred from free-form overlay names. Schema definitions govern required properties, legal endpoints, inherited fields and cardinalities. Source-defined stage-sensitive rules are checked only where applicable. Primary-key metadata does not impose additional global uniqueness.

Both implementations own a codec and semantic validator. They share this public format specification, without sharing production types or rules.
