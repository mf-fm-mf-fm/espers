use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"TNAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TNAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SunAndMoons {
    pub sunrise_begin: u8,
    pub sunrise_end: u8,
    pub sunset_begin: u8,
    pub sunset_end: u8,
    pub volatility: u8,
    pub moons: u8,
}

impl TryInto<SunAndMoons> for TNAM {
    type Error = Error;

    fn try_into(self) -> Result<SunAndMoons, Error> {
        Ok(SunAndMoons::read_le(&mut Cursor::new(&self.data))?)
    }
}
