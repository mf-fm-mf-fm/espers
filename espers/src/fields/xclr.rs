use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XCLR")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XCLR {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<XCLR> for Vec<FormID> {
    type Error = Error;

    fn try_from(raw: XCLR) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let mut result = Vec::new();
        while let Ok(fid) = FormID::read_le(&mut cursor) {
            result.push(fid);
        }
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
