use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{EDID, FLTV, FNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [GLOB](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/GLOB) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"GLOB")]
pub struct GLOB {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

/// Parsed [GLOB] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalVariable {
    pub header: RecordHeader,
    pub edid: String,
    pub kind: u8,
    pub value: f32,
}

impl fmt::Display for GlobalVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Global Variable ({})", self.edid)
    }
}

impl TryFrom<GLOB> for GlobalVariable {
    type Error = Error;

    fn try_from(raw: GLOB) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let kind = FNAM::read(&mut cursor)?.try_into()?;
        let value = FLTV::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            kind,
            value,
        })
    }
}
