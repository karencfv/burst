#![feature(asm)]

extern crate clap;

use std::rc::Rc;

use usdt::dtrace_provider;

use app::burst_app;

mod app;
mod client;

dtrace_provider!("src/burst.d");

#[tokio::main]
async fn main() {
    // Using Rc instead of Arc as none of the values are being mutated,
    // it's not necessary to pay the performance penalty.
    let client = Rc::new(burst_app());
    client.send_load().await;
}
