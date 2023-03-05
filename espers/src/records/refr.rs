use super::{get_cursor, Flags};
use crate::common::FormID;
use crate::error::Error;
use crate::fields::{EDID, NAME, VMAD};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"REFR")]
pub struct REFR {
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
pub struct ObjectRef {
    pub edid: Option<String>,
    pub name: FormID,
}

impl fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ObjectRef ({})",
            self.edid.as_ref().unwrap_or(&format!("{}", self.name)),
        )
    }
}

impl TryFrom<REFR> for ObjectRef {
    type Error = Error;

    fn try_from(raw: REFR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let _vmad = VMAD::read(&mut cursor);
        let name = NAME::read(&mut cursor)?.try_into()?;

        Ok(Self { edid, name })
    }
}
