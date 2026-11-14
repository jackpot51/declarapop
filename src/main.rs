use std::{ffi::OsString, io, process};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0} exited with {1}")]
    ExitStatus(String, process::ExitStatus),
    #[error("glib: {0}")]
    Glib(#[from] libflatpak::glib::Error),
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("from_utf8: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AptPackage(String);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FlatpakPackage(String);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub apt_packages: Vec<AptPackage>,
    //TODO: distinguish system and user?
    pub flatpak_packages: Vec<FlatpakPackage>,
}

fn command_stdout(mut command: process::Command) -> Result<String, Error> {
    let output = command.stderr(process::Stdio::inherit()).output()?;
    if !output.status.success() {
        return Err(Error::ExitStatus(
            command.get_program().to_string_lossy().into(),
            output.status,
        ));
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}

fn apt_packages() -> Result<Vec<AptPackage>, Error> {
    let mut command = process::Command::new("apt-mark");
    command.arg("showmanual");
    let stdout = command_stdout(command)?;
    Ok(stdout.lines().map(|x| AptPackage(x.into())).collect())
}

fn flatpak_packages() -> Result<Vec<FlatpakPackage>, Error> {
    use libflatpak::{Installation, gio::Cancellable, prelude::*};

    let inst = Installation::new_user(Cancellable::NONE)?;
    let refs = inst.list_installed_refs(Cancellable::NONE)?;
    Ok(refs
        .iter()
        .filter_map(|r| r.name().map(|x| FlatpakPackage(x.to_string())))
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apt_packages = apt_packages()?;
    let flatpak_packages = flatpak_packages()?;
    let config = Config {
        apt_packages,
        flatpak_packages,
    };
    println!("{}", toml::to_string_pretty(&config)?);
    Ok(())
}
