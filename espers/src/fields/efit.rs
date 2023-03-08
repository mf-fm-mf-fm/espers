use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"EFIT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EFIT {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectItem {
    pub magnitude: f32,
    pub area_of_effect: u32,
    pub duration: u32,
}

impl TryInto<EffectItem> for EFIT {
    type Error = Error;

    fn try_into(self) -> Result<EffectItem, Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(EffectItem::read_le(&mut cursor)?)
    }
}
