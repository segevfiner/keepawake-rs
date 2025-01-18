#[allow(unused_imports)]
use std::env;
use std::error::Error;

#[allow(unused_imports)]
use shadow_rs::ShadowBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "bin")]
    ShadowBuilder::builder().build()?;

    #[cfg(feature = "bin")]
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new()
            .set_manifest_file("keepawake.exe.manifest")
            .compile()?;
    }

    Ok(())
}
