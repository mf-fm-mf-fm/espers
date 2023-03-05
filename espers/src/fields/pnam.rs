use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"PNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PNAM {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<f32> for PNAM {
    type Error = Error;

    fn try_into(self) -> Result<f32, Error> {
        Ok(f32::read_le(&mut Cursor::new(&self.data))?)
    }
}
