use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"CNTO")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CNTO {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<CNTO> for (FormID, u32) {
    type Error = Error;

    fn try_from(raw: CNTO) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
