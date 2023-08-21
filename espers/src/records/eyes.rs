use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, LocalizedString};
use crate::error::Error;
use crate::fields::{DATA, EDID, FULL, ICON};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"EYES")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EYES {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
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
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eyes {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: LocalizedString,
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
        let full_name = if raw.localized {
            LocalizedString::Localized(FULL::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(FULL::read(&mut cursor)?.try_into()?)
        };
        let icon = ICON::read(&mut cursor)?.try_into()?;
        let flags = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            icon,
            flags,
        })
    }
}
