fn main() -> Result<(), Box<dyn std::error::Error>> {
    CPGSchema::Protogen::main(std::env::args().skip(1))?;
    Ok(())
}
