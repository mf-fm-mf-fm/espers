use crate::records::RawRecord;
use std::io;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    IOError(#[from] io::Error),

    #[error("Binary parse error: {}", _0)]
    BinaryParseError(#[from] binrw::Error),

    #[error("Binary parse error: {} - Record bytes: {:?}", _0, _1)]
    BinaryParseErrorExtra(binrw::Error, RawRecord),

    #[error("UTF-8 parse error")]
    Utf8ParseError(#[from] Utf8Error),

    #[error("ISO-8859-1 parse error")]
    ISO88591ParseError(u32),

    #[error("String EOF")]
    StringEOF,

    #[error("Extra bytes after parsing record ({:?})", _0)]
    ExtraBytes(Vec<u8>),

    #[error("Extra bytes after parsing record ({:?}) - Raw: ({:?})", _0, _1)]
    ExtraBytesRaw(Vec<u8>, RawRecord),

    #[error("Duplicate field encountered: ({})", _0)]
    DuplicateField(String),

    #[error("Unknown {} record version: {}", _0, _1)]
    UnknownVersion(String, u16),

    #[error("Duplicate String ID encountered: ({})", _0)]
    DuplicateStringID(u32),
}
