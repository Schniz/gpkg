use std::path::Path;
use std::process::{Command, Stdio};

pub fn install(cwd: impl AsRef<Path>) -> std::io::Result<()> {
    let status_code = if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", "npm install"])
            .current_dir(cwd)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    } else {
        Command::new("npm")
            .arg("install")
            .current_dir(cwd)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    };

    if status_code.success() {
        Ok(())
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
}
