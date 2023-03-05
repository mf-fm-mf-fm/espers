use super::{get_cursor, Flags};
use crate::error::Error;
use crate::fields::EDID;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"SLGM")]
pub struct SLGM {
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
pub struct SoulGem {
    pub edid: Option<String>,
}

impl fmt::Display for SoulGem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SoulGem ({})",
            self.edid.as_deref().unwrap_or("~")
        )
    }
}

impl TryFrom<SLGM> for SoulGem {
    type Error = Error;

    fn try_from(raw: SLGM) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self { edid })
    }
}
