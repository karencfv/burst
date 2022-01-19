#![cfg_attr(target_os = "macos", feature(asm_sym))]

extern crate clap;

use std::sync::Arc;

use usdt::dtrace_provider;

use app::burst_app;

mod app;
mod client;

dtrace_provider!("src/burst.d");

#[tokio::main]
async fn main() {
    let client = Arc::new(burst_app());
    client.send_load().await;
}
