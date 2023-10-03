use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, LocalizedString};
use crate::error::Error;
use crate::fields::{DATA, DESC, EDID, FULL, ICON};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"CLAS")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CLAS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassData {
    pub unknown: u32,
    pub training_skill: u8,
    pub training_level: u8,
    pub skill_weights: [u8; 18],
    pub bleedout_default: f32,
    pub voice_points: u32,
    pub health_weight: u8,
    pub magicka_weight: u8,
    pub stamina_weight: u8,
    pub flags: u8,
}

impl TryFrom<DATA> for ClassData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(Self::read(&mut cursor)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: LocalizedString,
    pub description: LocalizedString,
    pub icon: Option<String>,
    pub data: ClassData,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class ({})", self.edid)
    }
}

impl TryFrom<CLAS> for Class {
    type Error = Error;

    fn try_from(raw: CLAS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = if raw.localized {
            LocalizedString::Localized(FULL::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(FULL::read(&mut cursor)?.try_into()?)
        };
        let description = if raw.localized {
            LocalizedString::Localized(DESC::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(DESC::read(&mut cursor)?.try_into()?)
        };
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            description,
            icon,
            data,
        })
    }
}
