use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{DATA, DESC, EDID, FULL, ICON};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CLAS")]
pub struct CLAS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little)]
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
    pub full_name: String,
    pub description: String,
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
        let full_name = FULL::read(&mut cursor)?.try_into()?;
        let description = DESC::read(&mut cursor)?.try_into()?;
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let data = DATA::read(&mut cursor)?.try_into()?;

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
