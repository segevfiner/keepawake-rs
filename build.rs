use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    shadow_rs::new()?;

    if env::var("CARGO_FEATURE_BIN").is_ok() && env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new()
            .set_manifest_file("keepawake.exe.manifest")
            .compile()?;
    }

    Ok(())
}
