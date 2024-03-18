#![cfg(feature = "xtasks")]
mod commands;
mod task;

use commands::CARGO;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[command(subcommand)]
    // command: Command,
}

fn exit(exit_code: i32) -> ! {
    std::process::exit(exit_code)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let status = CARGO.clone().build().arg("build").spawn()?.wait()?;
    if let Some(exit_code) = status.code() {
        exit(exit_code);
    } else {
        return Err(anyhow::anyhow!("Failed to exit child process correctly"));
    }
}
