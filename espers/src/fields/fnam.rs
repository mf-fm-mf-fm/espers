use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"FNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<u32> for FNAM {
    type Error = Error;

    fn try_into(self) -> Result<u32, Error> {
        Ok(u32::read_le(&mut Cursor::new(&self.data))?)
    }
}

impl TryInto<String> for FNAM {
    type Error = Error;

    fn try_into(self) -> Result<String, Error> {
        Ok(NullString::read_le(&mut Cursor::new(&self.data))?.to_string())
    }
}
