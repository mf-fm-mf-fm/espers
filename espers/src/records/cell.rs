use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    DATA, EDID, FULL, LNAM, LTMP, MHDT, TVDT, XCAS, XCCM, XCIM, XCLC, XCLL, XCLR, XCLW, XCMO, XCWT,
    XEZN, XILL, XLCN, XNAM, XOWN, XWCN, XWCS, XWCU, XWEM,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [CELL](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/CELL) record
#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"CELL")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CELL {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CellFlags {
    U16(u16),
    U8(u8),
}

impl TryFrom<DATA> for CellFlags {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CellLocation {
    x: i32,
    y: i32,
    flags: u32,
}

impl TryFrom<XCLC> for CellLocation {
    type Error = Error;

    fn try_from(raw: XCLC) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CellLighting {
    pub ambient: [u8; 4],
    pub directional: [u8; 4],
    pub fog_near_color: [u8; 4],
    pub fog_near: f32,
    pub fog_far: f32,
    pub rotation_xy: [u8; 4],
    pub rotation_z: [u8; 4],
    pub directional_fade: f32,
    pub fog_clip_distance: f32,
    pub fog_pow: f32,
    pub ambient_x_plus: [u8; 4],
    pub ambient_x_minus: [u8; 4],
    pub ambient_y_plus: [u8; 4],
    pub ambient_y_minus: [u8; 4],
    pub ambient_z_plus: [u8; 4],
    pub ambient_z_minus: [u8; 4],
    #[br(try)]
    pub specular_color: Option<[u8; 4]>,
    #[br(try)]
    pub fresnel_power: Option<u32>,
    #[br(try)]
    pub fog_far_color: Option<[u8; 4]>,
    #[br(try)]
    pub fog_max: Option<f32>,
    #[br(try)]
    pub light_fade_dist_start: Option<f32>,
    #[br(try)]
    pub light_fade_dist_end: Option<f32>,
    #[br(try)]
    pub flags: Option<u32>,
}

impl TryFrom<XCLL> for CellLighting {
    type Error = Error;

    fn try_from(raw: XCLL) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<TVDT> for Vec<u8> {
    type Error = Error;

    fn try_from(raw: TVDT) -> Result<Self, Self::Error> {
        Ok(raw.data)
    }
}

impl TryFrom<MHDT> for Vec<u8> {
    type Error = Error;

    fn try_from(raw: MHDT) -> Result<Self, Self::Error> {
        Ok(raw.data)
    }
}
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaterCurrent {
    x: f32,
    y: f32,
    z: f32,
    unknown: f32,
}

impl TryFrom<XWCU> for Vec<WaterCurrent> {
    type Error = Error;

    fn try_from(raw: XWCU) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(raw.data);
        let mut result = Vec::new();
        while let Ok(fid) = WaterCurrent::read_le(&mut cursor) {
            result.push(fid);
        }
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [CELL] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub full_name: Option<LocalizedString>,
    pub flags: CellFlags,
    pub location: Option<CellLocation>,
    pub lighting: Option<CellLighting>,
    pub occlusion_data: Option<Vec<u8>>,
    pub max_height_data: Option<Vec<u8>>,
    pub lighting_template: FormID,
    pub lighting_template_flags: Option<u32>,
    pub water_height: f32,
    pub xnam: Option<u8>,
    pub containing_regions: Vec<FormID>,
    pub exit_location: Option<FormID>,
    pub water_current_count_3: Option<u32>,
    pub water_current_count_4: Option<u32>,
    pub water_current: Vec<WaterCurrent>,
    pub water: Option<FormID>,
    pub owner: Option<FormID>,
    pub lock_list: Option<FormID>,
    pub water_environment_map: Option<String>,
    pub climate: Option<FormID>,
    pub acoustic_space: Option<FormID>,
    pub encounter_zone: Option<FormID>,
    pub music_type: Option<FormID>,
    pub image_space: Option<FormID>,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<CELL> for Cell {
    type Error = Error;

    fn try_from(raw: CELL) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };

        let flags = DATA::read(&mut cursor)?.try_into()?;
        let location = XCLC::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let lighting = XCLL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let occlusion_data = TVDT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let max_height_data = MHDT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let lighting_template = LTMP::read(&mut cursor)?.try_into()?;
        let lighting_template_flags = LNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let water_height = XCLW::read(&mut cursor)?.try_into()?;
        let xnam = XNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let containing_regions = match XCLR::read(&mut cursor) {
            Ok(r) => r.try_into()?,
            Err(_) => Vec::new(),
        };

        let mut exit_location = None;
        let mut water_current_count_3 = None;
        let mut water_current_count_4 = None;
        let mut water_current = Vec::new();
        let mut water = None;
        let mut owner = None;
        let mut lock_list = None;
        let mut water_environment_map = None;
        let mut climate = None;
        let mut acoustic_space = None;
        let mut encounter_zone = None;
        let mut music_type = None;
        let mut image_space = None;

        loop {
            if let Ok(x) = XLCN::read(&mut cursor) {
                exit_location = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XWCS::read(&mut cursor) {
                water_current_count_3 = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XWCN::read(&mut cursor) {
                water_current_count_4 = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XWCU::read(&mut cursor) {
                let items: Vec<_> = x.try_into()?;
                water_current.extend(items);
                continue;
            }
            if let Ok(x) = XCWT::read(&mut cursor) {
                water = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XOWN::read(&mut cursor) {
                owner = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XILL::read(&mut cursor) {
                lock_list = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XWEM::read(&mut cursor) {
                water_environment_map = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XCCM::read(&mut cursor) {
                climate = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XCAS::read(&mut cursor) {
                acoustic_space = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XEZN::read(&mut cursor) {
                encounter_zone = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XCMO::read(&mut cursor) {
                music_type = Some(x.try_into()?);
                continue;
            }
            if let Ok(x) = XCIM::read(&mut cursor) {
                image_space = Some(x.try_into()?);
                continue;
            }
            break;
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            flags,
            location,
            lighting,
            occlusion_data,
            max_height_data,
            lighting_template,
            lighting_template_flags,
            water_height,
            xnam,
            containing_regions,
            exit_location,
            water_current_count_3,
            water_current_count_4,
            water_current,
            water,
            owner,
            lock_list,
            water_environment_map,
            climate,
            acoustic_space,
            encounter_zone,
            music_type,
            image_space,
        })
    }
}
