use {
    std::{env, io},
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    // Add Icon to Executable
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("./assets/icons/watchdog-logo.ico")
            .compile()?;
    }

    Ok(())
}
