use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{Model, ObjectBounds, DATA, EDID, MODL, MODS, MODT, OBND};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [GRAS](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/GRAS) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"GRAS")]
pub struct GRAS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct GrassFlags: u8 {
        const VERTEX_LIGHTING = 1;
        const UNIFORM_SCALING = 2;
        const FIT_TO_SLOPE = 4;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrassData {
    pub density: u8,
    pub min_slope: u8,
    pub max_slope: u8,
    pub unused: u8,
    pub distance_from_water: u16,
    pub unused_2: u16,
    pub water_distance_flags: u32,
    pub position_range: f32,
    pub height_range: f32,
    pub color_range: f32,
    pub wave_period: f32,
    pub flags: GrassFlags,
    pub unused_3: u8,
    pub unused_4: u16,
}

impl TryFrom<DATA> for GrassData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [GRAS] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grass {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub model: Option<Model>,
    pub data: GrassData,
}

impl fmt::Display for Grass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grass ({})", self.edid)
    }
}

impl TryFrom<GRAS> for Grass {
    type Error = Error;

    fn try_from(raw: GRAS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let model = Model::try_load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;
        let data = DATA::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            model,
            data,
        })
    }
}
