use std::{
    io,
    process::{self, Command},
    sync::mpsc::channel,
};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

use keepawake::{Awake, AwakeOptions};

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

    /// Generate shell completions
    #[arg(long, value_enum, value_name("SHELL"))]
    completions: Option<Shell>,

    /// Run the command and wait for it to exit, keeping the computer awake while it runs.
    #[arg()]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        generate(shell, &mut Cli::command(), "keepawake", &mut io::stdout());
        return Ok(());
    }

    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    let exit_code = {
        let _awake = Awake::new(&AwakeOptions {
            display: cli.display,
            idle: cli.idle,
            sleep: cli.sleep,
        })?;

        if cli.command.len() > 0 {
            // TODO Improve exit code in signal
            Command::new(&cli.command[0])
                .args(&cli.command[1..])
                .spawn()?
                .wait()?
                .code()
                .unwrap_or(128)
        } else {
            rx.recv().expect("Could not receive from channel.");
            130
        }
    };

    process::exit(exit_code);
}
