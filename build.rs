use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    shadow_rs::new()?;

    if cfg!(all(feature = "bin", target_os = "windows")) {
        winres::WindowsResource::new()
            .set_manifest_file("keepawake.exe.manifest")
            .compile()?;
    }

    Ok(())
}
