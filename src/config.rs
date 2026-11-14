use std::{collections::BTreeMap, process};

use crate::{Error, util::command_stdout};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AptPackage(pub String);

fn apt_packages() -> Result<Vec<AptPackage>, Error> {
    let mut command = process::Command::new("apt-mark");
    command.arg("showmanual");
    let stdout = command_stdout(command)?;
    Ok(stdout.lines().map(|x| AptPackage(x.into())).collect())
}

fn language_packages(lang: &str) -> Result<Vec<AptPackage>, Error> {
    let mut command = process::Command::new("check-language-support");
    command.arg("-l").arg(lang).arg("--show-installed");
    let stdout = command_stdout(command)?;
    Ok(stdout.split(" ").map(|x| AptPackage(x.into())).collect())
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
    pub languages: BTreeMap<String, Vec<AptPackage>>,
    //TODO: distinguish system and user?
    pub flatpak_packages: Vec<FlatpakPackage>,
}

impl Config {
    pub fn current() -> Result<Self, Error> {
        let apt_packages = apt_packages()?;
        let mut languages = BTreeMap::new();
        for p in apt_packages.iter() {
            if let Some(lang) = p.0.strip_prefix("language-pack-") {
                let len = lang.len();
                // check-language-support supports two or three code languages, or the Chinese simplified/traditional codes
                if len == 2 || len == 3 || lang == "zh-hans" || lang == "zh-hant" {
                    match language_packages(lang) {
                        Ok(packages) => {
                            //TODO: return error or combine packages if there is a duplicate?
                            assert!(languages.insert(lang.to_string(), packages).is_none());
                        }
                        Err(err) => {
                            eprintln!("failed to get languages for {}: {}", lang, err);
                        }
                    }
                }
            }
        }
        let flatpak_packages = flatpak_packages()?;
        Ok(Config {
            apt_packages,
            languages,
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
