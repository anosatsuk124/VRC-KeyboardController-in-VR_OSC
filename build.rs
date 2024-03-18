use anyhow::Result;

#[cfg(not(target_os = "windows"))]
fn main() -> Result<()> {
    env_logger::init();
    Ok(())
}

#[cfg(target_os = "windows")]
fn main() -> Result<()> {
    env_logger::init();

    log::info!("Started to build for Windows");

    let mut res = winresource::WindowsResource::new();
    // TODO: Prepare any icon.
    // res.set_icon("test.ico");
    res.compile()?;

    Ok(())
}
