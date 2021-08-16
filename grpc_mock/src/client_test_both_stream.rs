mod client_main;

use client_main::run_test_both_stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_test_both_stream().await?;
    Ok(())
}