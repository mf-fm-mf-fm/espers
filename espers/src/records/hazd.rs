use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{Model, ObjectBounds, DATA, EDID, FULL, MNAM, MODL, MODS, MODT, OBND};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [HAZD](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/HAZD) record
#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"HAZD")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HAZD {
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
    pub struct HazardFlags: u32 {
        const AFFECTS_PLAYER_ONLY = 0x01;
        const INHERIT_DURATION = 0x02;
        const ALIGN_IMPACT_NORMAL = 0x04;
        const INHERIT_RADIUS = 0x08;
        const DROP_TO_GROUND = 0x10;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HazardData {
    pub limit: u32,
    pub radius: f32,
    pub lifetime: f32,
    pub is_radius: f32,
    pub target_interval: f32,
    pub flags: HazardFlags,
    pub spell: FormID,
    pub light: FormID,
    pub impact_data_set: FormID,
    pub sound: FormID,
}

impl TryFrom<DATA> for HazardData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading::<_>(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [HAZD] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hazard {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model: Option<Model>,
    pub image_space_mod: Option<FormID>,
    pub data: HazardData,
}

impl fmt::Display for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hazard ({})", self.edid)
    }
}

impl TryFrom<HAZD> for Hazard {
    type Error = Error;

    fn try_from(raw: HAZD) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);
        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let model = Model::try_load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;
        let image_space_mod = MNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            full_name,
            model,
            image_space_mod,
            data,
        })
    }
}
