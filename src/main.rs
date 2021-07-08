#![feature(asm)]
#[macro_use]
extern crate clap;

use usdt::dtrace_provider;

use app::burst_app;

mod app;
mod client;

dtrace_provider!("src/burst.d");

#[tokio::main]
async fn main() {
    let client = burst_app();
    client.send_load().await;
}
