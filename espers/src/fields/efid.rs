use crate::common::FormID;
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"EFID")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EFID {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<FormID> for EFID {
    type Error = Error;

    fn try_into(self) -> Result<FormID, Error> {
        Ok(FormID::read_le(&mut Cursor::new(&self.data))?)
    }
}
