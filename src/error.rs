#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("enum conversion: {0}")]
    Strum(#[from] strum::ParseError),
    #[error("utf8 conversion: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("utf16 conversion: {0}")]
    Utf16(#[from] std::string::FromUtf16Error),
    #[error("bufwriter dereference: {0}")]
    IntoInner(#[from] std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),
    #[error("found magic of {0:#x} instead of {:#x}", super::MAGIC)]
    Magic(u32),
    #[error("used version {0} but pak is version {1}")]
    Version(super::Version, super::Version),
    #[error("got {0}, which is not a boolean")]
    Bool(u8),
    #[error("{0}")]
    Other(String),
}
