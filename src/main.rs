use std::{error::Error, io};

use awake_rs::{AwakeOptions, Awake};

fn main() -> Result<(), Box<dyn Error>> {
    let _awake = Awake::new(&AwakeOptions{
        idle: true,
        ..Default::default()
    })?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(())
}
