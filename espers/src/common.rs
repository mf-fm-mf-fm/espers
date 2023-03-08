use crate::error::Error;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::{Read, Seek};

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormID(pub u32);

impl fmt::Display for FormID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FormID({:#010X})", self.0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LocalizedString {
    Localized(u32),
    ZString(String),
}

impl fmt::Display for LocalizedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocalizedString::Localized(l) => write!(f, "LocalizedString::Localized({:?})", l),
            LocalizedString::ZString(z) => write!(f, "LocalizedString::ZString({})", z),
        }
    }
}

pub fn check_done_reading<T: Read + Seek>(reader: &mut T) -> Result<(), Error> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    if buf.is_empty() {
        Ok(())
    } else {
        Err(Error::ExtraBytes(buf))
    }
}
