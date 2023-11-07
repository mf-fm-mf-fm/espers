use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

use crate::{common::check_done_reading, error::Error};

#[binrw]
#[brw(little, magic = b"XMRK")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XMRK {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<XMRK> for String {
    type Error = Error;

    fn try_from(raw: XMRK) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
