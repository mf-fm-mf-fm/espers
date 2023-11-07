use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, until_eof, BinRead, Endian};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"KWDA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KWDA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<KWDA> for u32 {
    type Error = Error;

    fn try_from(raw: KWDA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<KWDA> for Vec<FormID> {
    type Error = Error;

    fn try_from(raw: KWDA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = until_eof(&mut cursor, Endian::Little, ())?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
