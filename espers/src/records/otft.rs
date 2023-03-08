use super::{get_cursor, Flags, RecordHeader};
use crate::common::FormID;
use crate::error::Error;
use crate::fields::{EDID, INAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"OTFT")]
pub struct OTFT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outfit {
    pub header: RecordHeader,
    pub edid: String,
    pub form_ids: Vec<FormID>,
}

impl fmt::Display for Outfit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Outfit ({})", self.edid)
    }
}

impl TryFrom<OTFT> for Outfit {
    type Error = Error;

    fn try_from(raw: OTFT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let form_ids = INAM::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            form_ids,
        })
    }
}
