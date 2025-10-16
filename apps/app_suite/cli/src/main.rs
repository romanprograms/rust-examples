use anyhow::Result;

fn main() -> Result<()> {
    let name = std::env::args().nth(1).unwrap_or_else(|| "world".into());
    println!("{}", core_utils::greet(&name));
    Ok(())
}