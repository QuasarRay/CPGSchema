//! Schema-driven protobuf declaration export, following Flatgraph 0.1.36 ProtoGen.
use crate::model::{Error, ProtoOptions, Result, Schema, SchemaBuilder};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
pub fn options() -> ProtoOptions {
    ProtoOptions {
        pkg: "cpg".into(),
        java_outer_classname: "Cpg".into(),
        java_package: "io.shiftleft.proto.cpg".into(),
        go_package: "github.com/ShiftLeftSecurity/proto/cpg".into(),
        csharp_namespace: "io.shiftleft.proto.cpg".into(),
        enum_names: BTreeMap::from([
            ("Languages".into(), "LANGUAGES".into()),
            (
                "ControlStructureTypes".into(),
                "CONTROL_STRUCTURE_TYPES".into(),
            ),
            ("Frameworks".into(), "FRAMEWORKS".into()),
        ]),
    }
}
pub fn schema() -> Result<Schema> {
    let mut b = SchemaBuilder::new("Cpg", "io.shiftleft.codepropertygraph.generated", &[]);
    b.set_proto_options(options());
    crate::CpgSchema::CpgSchema::new(&mut b)?;
    b.build()
}
fn entries<'a>(
    values: impl IntoIterator<Item = (&'a str, Option<i32>, Option<&'a str>)>,
) -> String {
    let mut values: Vec<_> = values
        .into_iter()
        .filter_map(|(n, id, c)| id.map(|i| (n, i, c)))
        .collect();
    values.sort_by_key(|(_, i, _)| *i);
    values
        .into_iter()
        .map(|(n, i, c)| {
            format!(
                "  {}\n  {n} = {i};\n",
                c.map(|c| format!("/* {} */", c.replace("*/", "* /")))
                    .unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
fn singular(name: &str) -> String {
    if let Some(s) = name.strip_suffix("ies") {
        format!("{s}y")
    } else {
        name.strip_suffix('s').unwrap_or(name).into()
    }
}
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_uppercase());
    }
    out
}
pub fn generate(schema: &Schema) -> Result<String> {
    let opts = schema
        .proto_options
        .as_ref()
        .ok_or_else(|| Error::new("Protogen", "schema has no protobuf options"))?;
    let mut constants = String::new();
    for cat in &schema.constants {
        if cat.entries.iter().any(|c| c.proto_id.is_some()) {
            let name = opts.enum_names.get(&cat.name).unwrap_or(&cat.name);
            constants.push_str(&format!(
                "enum {name} {{\n  UNKNOWN_{} = 0;\n\n  {}\n}}\n\n",
                snake(&singular(&cat.name)),
                entries(cat.entries.iter().map(|c| (
                    c.name.as_str(),
                    c.proto_id,
                    c.comment.as_deref()
                )))
            ));
        }
    }
    let np = entries(schema.node_properties()?.iter().map(|id| {
        let p = &schema.properties[id.0];
        (p.name.as_str(), p.proto_id, p.comment.as_deref())
    }));
    let ep = entries(schema.edge_properties().iter().map(|id| {
        let p = &schema.properties[id.0];
        (p.name.as_str(), p.proto_id, p.comment.as_deref())
    }));
    let nt = entries(
        schema
            .concrete_nodes()
            .into_iter()
            .map(|(_, n)| (n.name.as_str(), n.proto_id, n.comment.as_deref())),
    );
    let et = entries(
        schema
            .edges
            .iter()
            .map(|e| (e.name.as_str(), e.proto_id, e.comment.as_deref())),
    );
    let replacements = [
        ("${protoOpts.pkg}", opts.pkg.as_str()),
        ("${protoOpts.goPackage}", opts.go_package.as_str()),
        ("${protoOpts.javaPackage}", opts.java_package.as_str()),
        (
            "${protoOpts.javaOuterClassname}",
            opts.java_outer_classname.as_str(),
        ),
        (
            "${protoOpts.csharpNamespace}",
            opts.csharp_namespace.as_str(),
        ),
        (
            "${protoDefs(schema.nodeProperties.map(enumEntryMaybe))}",
            np.as_str(),
        ),
        (
            "${protoDefs(schema.edgeProperties.map(enumEntryMaybe))}",
            ep.as_str(),
        ),
        (
            "${protoDefs(schema.nodeTypes.map(enumEntryMaybe))}",
            nt.as_str(),
        ),
        (
            "${protoDefs(schema.edgeTypes.map(enumEntryMaybe))}",
            et.as_str(),
        ),
        ("$enumsFromConstants", constants.as_str()),
    ];
    let mut out = include_str!("layout.proto").to_owned();
    for (from, to) in replacements {
        out = out.replace(from, to);
    }
    Ok(out)
}
pub fn run(schema: &Schema, directory: &Path) -> Result<PathBuf> {
    let opts = schema
        .proto_options
        .as_ref()
        .ok_or_else(|| Error::new("Protogen", "schema has no protobuf options"))?;
    let text = generate(schema)?;
    std::fs::create_dir_all(directory)
        .map_err(|e| Error::new(directory.display().to_string(), e.to_string()))?;
    let path = directory.join(format!("{}.proto", opts.pkg));
    std::fs::write(&path, text)
        .map_err(|e| Error::new(path.display().to_string(), e.to_string()))?;
    Ok(path)
}
pub fn main(args: impl IntoIterator<Item = String>) -> Result<PathBuf> {
    let dir = args
        .into_iter()
        .next()
        .ok_or_else(|| Error::new("Protogen", "please pass outputDir as first parameter"))?;
    run(&schema()?, Path::new(&dir))
}
