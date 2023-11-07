use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

use crate::{common::check_done_reading, error::Error};

#[binrw]
#[brw(little, magic = b"CSMD")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CSMD {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<CSMD> for (f32, f32) {
    type Error = Error;

    fn try_from(raw: CSMD) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
