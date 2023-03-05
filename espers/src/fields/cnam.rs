use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"CNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<RGBA8> for CNAM {
    type Error = Error;

    fn try_into(self) -> Result<RGBA8, Self::Error> {
        let parsed: [u8; 4] = BinRead::read(&mut Cursor::new(&self.data))?;
        Ok(parsed.into())
    }
}

impl TryInto<String> for CNAM {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}
