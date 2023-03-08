use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"CTDA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CTDA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub operator: u8,
    pub unknown1: [u8; 3],
    pub comparison_value: f32,
    pub function_index: u16,
    pub padding: u16,
    pub param1: i32,
    pub param2: i32,
    // pub param1: u16,
    // pub param2: [u8; 2],
    // pub param3: u32,
    pub run_on_type: u32,
    pub reference: u32,
    pub unknown2: i32,
}

impl TryInto<Condition> for CTDA {
    type Error = Error;

    fn try_into(self) -> Result<Condition, Self::Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(Condition::read(&mut cursor)?)
    }
}
