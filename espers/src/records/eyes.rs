use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{DATA, EDID, FULL, ICON};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"EYES")]
pub struct EYES {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct EyesFlags: u8 {
        const PLAYABLE = 0x01;
        const NOT_MALE = 0x02;
        const NOT_FEMALE = 0x04;
    }
}

impl TryFrom<DATA> for EyesFlags {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(Self::read(&mut cursor)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eyes {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: String,
    pub icon: String,
    pub flags: EyesFlags,
}

impl fmt::Display for Eyes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Eyes ({})", self.edid)
    }
}

impl TryFrom<EYES> for Eyes {
    type Error = Error;

    fn try_from(raw: EYES) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = FULL::read(&mut cursor)?.try_into()?;
        let icon = ICON::read(&mut cursor)?.try_into()?;
        let flags = DATA::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            icon,
            flags,
        })
    }
}
