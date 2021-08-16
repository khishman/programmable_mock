use std::env;
mod client_main;

use client_main::run_basic_req_resp_test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_basic_req_resp_test().await?;
    Ok(())
}