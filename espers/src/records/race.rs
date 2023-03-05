use super::{get_cursor, Flags};
use crate::error::Error;
use crate::fields::{DESC, EDID, FULL};
use binrw::binrw;
use binrw::BinRead;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"RACE")]
pub struct RACE {
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
pub struct Race {
    pub edid: String,
    pub full_name: Option<String>,
    pub description: String,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Race ({})", self.edid)
    }
}

impl TryFrom<RACE> for Race {
    type Error = Error;

    fn try_from(raw: RACE) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = FULL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let description = DESC::read(&mut cursor)?.try_into()?;

        Ok(Self {
            edid,
            full_name,
            description,
        })
    }
}
