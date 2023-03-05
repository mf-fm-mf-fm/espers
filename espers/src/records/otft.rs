use super::{get_cursor, Flags};
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
    pub size: u32,
    pub flags: Flags,
    pub form_id: u32,
    pub timestamp: u16,
    pub version_control: u16,
    pub internal_version: u16,
    pub unknown: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outfit {
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
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let form_ids = INAM::read(&mut cursor)?.try_into()?;

        Ok(Self { edid, form_ids })
    }
}
