use crate::Error;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AptPackage(pub String);

fn apt_packages() -> Result<Vec<AptPackage>, Error> {
    let mut command = std::process::Command::new("apt-mark");
    command.arg("showmanual");
    let stdout = crate::util::command_stdout(command)?;
    Ok(stdout.lines().map(|x| AptPackage(x.into())).collect())
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FlatpakPackage(pub String);

pub fn flatpak_packages() -> Result<Vec<FlatpakPackage>, Error> {
    use libflatpak::{Installation, RefKind, gio::Cancellable, prelude::*};

    let inst = Installation::new_user(Cancellable::NONE)?;
    //TODO: also include pinned runtimes?
    let refs = inst.list_installed_refs_by_kind(RefKind::App, Cancellable::NONE)?;
    Ok(refs
        .iter()
        .filter_map(|r| r.name().map(|x| FlatpakPackage(x.to_string())))
        .collect())
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub apt_packages: Vec<AptPackage>,
    //TODO: distinguish system and user?
    pub flatpak_packages: Vec<FlatpakPackage>,
}

impl Config {
    pub fn current() -> Result<Self, Error> {
        let apt_packages = apt_packages()?;
        let flatpak_packages = flatpak_packages()?;
        Ok(Config {
            apt_packages,
            flatpak_packages,
        })
    }

    pub fn as_toml(&self) -> Result<String, Error> {
        let toml = toml::to_string_pretty(self)?;
        Ok(toml)
    }

    pub fn from_toml(toml: &str) -> Result<Self, Error> {
        let this: Self = toml::from_str(toml)?;
        Ok(this)
    }
}
