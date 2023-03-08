use std::io;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O Error")]
    IOError(#[from] io::Error),

    #[error("Binary Parse Error")]
    BinaryParseError(#[from] binrw::Error),

    #[error("UTF-8 Parse Error")]
    Utf8ParseError(#[from] Utf8Error),

    #[error("ISO-8859-1 Parse Error")]
    ISO88591ParseError(u32),

    #[error("String EOF")]
    StringEOF,

    #[error("Extra bytes after parsing record")]
    ExtraBytes(Vec<u8>),
}
