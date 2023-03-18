use crate::{common::check_done_reading, error::Error};
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
    pub unknown5_count: u32,

    #[br(count = unknown4_count)]
    pub unknown4s: Vec<Unknown4>,

    #[br(count = unknown5_count)]
    pub unknown5: Vec<u32>,
}

impl TryInto<ModelTextures> for MODT {
    type Error = Error;

    fn try_into(self) -> Result<ModelTextures, Error> {
        Ok(ModelTextures::read(&mut Cursor::new(&self.data))?)
    }
}

impl TryFrom<MODT> for Vec<Unknown4> {
    type Error = Error;

    fn try_from(raw: MODT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let mut parsed = Vec::new();
        while let Ok(result) = Unknown4::read_le(&mut cursor) {
            parsed.push(result);
        }
        check_done_reading(&mut cursor)?;
        Ok(parsed)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Textures {
    Header(ModelTextures),
    NoHeader(Vec<Unknown4>),
}

impl Textures {
    pub fn load(modt: MODT, version: u16) -> Result<Self, Error> {
        Ok(match version {
            0..=37 => Err(Error::UnknownVersion(format!("{:?}", modt), version))?,
            38..=39 => Self::NoHeader(modt.try_into()?),
            40.. => Self::Header(modt.try_into()?),
        })
    }
}
