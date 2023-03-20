use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"ENIT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ENIT {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnchantedItem {
    pub cost: u32,
    pub flags: u32,
    pub cast_type: u32,
    pub amount: u32,
    pub delivery: u32,
    pub kind: u32,
    pub charge_time: f32,
    pub base_enchantment: u32,
    #[br(try)]
    pub worn_restrictions: Option<u32>,
}

impl TryFrom<ENIT> for EnchantedItem {
    type Error = Error;

    fn try_from(raw: ENIT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
