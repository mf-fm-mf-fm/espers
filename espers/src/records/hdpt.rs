use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    Model, CNAM, DATA, EDID, FULL, HNAM, MODL, MODS, MODT, NAM0, NAM1, PNAM, RNAM, TNAM,
};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [HDPT](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/HDPT) record
#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"HDPT")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDPT {
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
    pub struct HeadPartFlags: u8 {
        const PLAYABLE = 0x01;
        const MALE = 0x02;
        const FEMALE = 0x04;
        const IS_EXTRA_PART = 0x08;
        const USE_SOLID_TINT = 0x10;
    }
}

impl TryFrom<DATA> for HeadPartFlags {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[brw(little, repr = u32)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeadPartType {
    Misc = 0,
    Face,
    Eyes,
    Hair,
    FacialHair,
    Scar,
    Eyebrows,
}

impl TryFrom<PNAM> for HeadPartType {
    type Error = Error;

    fn try_from(raw: PNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [HDPT] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadPart {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<LocalizedString>,
    pub model: Option<Model>,
    pub flags: HeadPartFlags,
    pub kind: HeadPartType,
    pub additional_parts: Vec<FormID>,
    pub options: Vec<(u32, String)>,
    pub base_texture: Option<FormID>,
    pub color: Option<FormID>,
    pub resources: Option<FormID>,
}

impl fmt::Display for HeadPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HeadPart ({})", self.edid)
    }
}

impl TryFrom<HDPT> for HeadPart {
    type Error = Error;

    fn try_from(raw: HDPT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let model = Model::try_load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;
        let flags = DATA::read(&mut cursor)?.try_into()?;
        let kind = PNAM::read(&mut cursor)?.try_into()?;
        let mut additional_parts = Vec::new();
        while let Ok(ap) = HNAM::read(&mut cursor) {
            additional_parts.push(ap.try_into()?);
        }
        let mut options = Vec::new();
        while let Ok(x) = NAM0::read(&mut cursor) {
            let y = NAM1::read(&mut cursor)?.try_into()?;
            options.push((x.try_into()?, y));
        }
        let base_texture = TNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let color = CNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let resources = RNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            model,
            flags,
            kind,
            additional_parts,
            options,
            base_texture,
            color,
            resources,
        })
    }
}
