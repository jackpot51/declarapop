use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0} exited with {1}")]
    ExitStatus(String, std::process::ExitStatus),
    #[error("from_utf8: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("glib: {0}")]
    Glib(#[from] libflatpak::glib::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml deserialize: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialize: {0}")]
    TomlSer(#[from] toml::ser::Error),
}
