use std::{error::Error, io};

use keepawake::{AwakeOptions, Awake};

fn main() -> Result<(), Box<dyn Error>> {
    // TODO CLI
    let _awake = Awake::new(&AwakeOptions{
        display: true,
        idle: true,
        ..Default::default()
    })?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(())
}
