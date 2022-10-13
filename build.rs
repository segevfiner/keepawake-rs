use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    shadow_rs::new().map_err(|err| err.to_string())?;

    if cfg!(target_os = "windows") {
        winres::WindowsResource::new().set_manifest_file("keepawake.exe.manifest").compile()?;
    }

    Ok(())
}
