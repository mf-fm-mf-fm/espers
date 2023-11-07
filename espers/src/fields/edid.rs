use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, io::Cursor, BinRead, BinWrite, NullString};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"EDID")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EDID {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<EDID> for String {
    type Error = Error;

    fn try_from(raw: EDID) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = NullString::read_le(&mut cursor)?.to_string();
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<String> for EDID {
    type Error = Error;

    fn try_from(obj: String) -> Result<EDID, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        NullString::from(obj).write(&mut cursor)?;
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
