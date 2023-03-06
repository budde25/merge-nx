use camino::Utf8PathBuf;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    #[error("File in path `{0}` does not exist")]
    FileDoesNotExist(Utf8PathBuf),

    #[error("Invaid extension `{0}` expected ns* or xc*")]
    InvalidExtension(String),
}
