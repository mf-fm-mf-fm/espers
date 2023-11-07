use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{CNAM, DNAM, EDID, ENAM, HNAM, TNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"IMGS")]
pub struct IMGS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSpace {
    pub header: RecordHeader,
    pub edid: String,
}

impl fmt::Display for ImageSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImageSpace ({})", self.edid)
    }
}

impl TryFrom<IMGS> for ImageSpace {
    type Error = Error;

    fn try_from(raw: IMGS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let _ = ENAM::read(&mut cursor).ok();
        let _ = HNAM::read(&mut cursor).ok();
        let _ = CNAM::read(&mut cursor).ok();
        let _ = TNAM::read(&mut cursor).ok();
        let _ = DNAM::read(&mut cursor).ok();

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
        })
    }
}
