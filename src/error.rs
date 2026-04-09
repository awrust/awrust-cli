#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Http(#[from] reqwest::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("xml: {0}")]
    Xml(String),
    #[error("{0}: {1}")]
    Api(u16, String),
}

pub type Result<T> = std::result::Result<T, Error>;
