use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"BPND")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BPND {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl<const N: usize> TryFrom<BPND> for [u32; N] {
    type Error = Error;

    fn try_from(raw: BPND) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
