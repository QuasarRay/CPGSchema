fn main() -> Result<(), Box<dyn std::error::Error>> {
    cpg_schema_json::Protogen::main(std::env::args().skip(1))?;
    Ok(())
}
