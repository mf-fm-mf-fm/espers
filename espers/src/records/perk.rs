use super::{get_cursor, Flags};
use crate::error::Error;
use crate::fields::{Script, DESC, EDID, FULL, ICON, VMAD};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"PERK")]
pub struct PERK {
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
pub struct Perk {
    pub edid: String,
    pub scripts: Vec<Script>,
    pub full_name: Option<String>,
    pub description: String,
    pub icon: Option<String>,
}

impl fmt::Display for Perk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Perk ({})", self.edid)
    }
}

impl TryFrom<PERK> for Perk {
    type Error = Error;

    fn try_from(raw: PERK) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?
            .unwrap_or_default();
        let full_name = FULL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let description = DESC::read(&mut cursor)?.try_into()?;
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            edid,
            scripts,
            full_name,
            description,
            icon,
        })
    }
}
