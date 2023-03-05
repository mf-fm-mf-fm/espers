use super::{get_cursor, Flags};
use crate::error::Error;
use crate::fields::{EDID, FULL, PNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"DIAL")]
pub struct DIAL {
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
pub struct DialogueTopic {
    pub edid: Option<String>,
    pub full_name: Option<String>,
    pub priority: f32,
}

impl fmt::Display for DialogueTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DialogueTopic ({})",
            self.edid.as_deref().unwrap_or("~")
        )
    }
}

impl TryFrom<DIAL> for DialogueTopic {
    type Error = Error;

    fn try_from(raw: DIAL) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let full_name = FULL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let priority = PNAM::read(&mut cursor)?.try_into()?;

        Ok(Self {
            edid,
            full_name,
            priority,
        })
    }
}
