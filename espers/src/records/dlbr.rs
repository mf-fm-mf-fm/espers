use super::{get_cursor, Flags};
use crate::error::Error;
use crate::fields::{EDID, QNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"DLBR")]
pub struct DLBR {
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
pub struct DialogueBranch {
    pub edid: String,
    pub quest_id: u32,
}

impl fmt::Display for DialogueBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DialogBranch ({}) {:X}", self.edid, self.quest_id)
    }
}

impl TryFrom<DLBR> for DialogueBranch {
    type Error = Error;

    fn try_from(raw: DLBR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let quest_id = QNAM::read(&mut cursor)?.try_into()?;

        Ok(Self { edid, quest_id })
    }
}
