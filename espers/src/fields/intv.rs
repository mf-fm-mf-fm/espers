use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"INTV")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct INTV {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<u32> for INTV {
    type Error = Error;

    fn try_into(self) -> Result<u32, Error> {
        Ok(u32::read_le(&mut Cursor::new(&self.data))?)
    }
}
