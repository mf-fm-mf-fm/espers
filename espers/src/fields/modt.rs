use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"MODT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MODT {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Unknown4 {
    pub unknown1: u32,
    pub unknown2: [u8; 4],
    pub unknown3: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelTextures {
    pub count: u32,
    pub unknown4_count: u32,
    pub unknown2: u32,

    #[br(count = unknown4_count)]
    pub unknown4s: Vec<Unknown4>,
}

impl TryInto<ModelTextures> for MODT {
    type Error = Error;

    fn try_into(self) -> Result<ModelTextures, Error> {
        Ok(ModelTextures::read(&mut Cursor::new(&self.data))?)
    }
}
