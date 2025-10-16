use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Call into shared libraries
    let msg = core_utils::greet("workspace");
    println!("{}", msg);

    // Show DB connect string determined by feature flags
    println!("DB connect: {}", db::connect_info());

    // Pretend to do async work
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    Ok(())
}
