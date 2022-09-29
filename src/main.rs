use std::{error::Error, sync::mpsc::channel};

use keepawake::{Awake, AwakeOptions};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Keep display on
    #[arg(short, long)]
    display: bool,

    /// Keep system from idle sleeping
    #[arg(short, long)]
    idle: bool,

    /// Keep system from sleeping (Functionality and conditions for this to work vary by OS)
    #[arg(short, long)]
    sleep: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    let _awake = Awake::new(&AwakeOptions {
        display: cli.display,
        idle: cli.idle,
        sleep: cli.sleep,
    })?;

    rx.recv().expect("Could not receive from channel.");

    Ok(())
}
