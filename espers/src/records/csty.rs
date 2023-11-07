use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{CSCR, CSFL, CSGD, CSLR, CSMD, CSME, DATA, EDID};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [CSTY](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/CSTY) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CSTY")]
pub struct CSTY {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralStyle {
    pub offensive_mult: f32,
    pub defensive_mult: f32,
    #[br(try)]
    pub group_offensive_mult: Option<f32>,
    #[br(try)]
    pub melee_equipment_mult: Option<f32>,
    #[br(try)]
    pub magic_equipment_mult: Option<f32>,
    #[br(try)]
    pub ranged_equipment_mult: Option<f32>,
    #[br(try)]
    pub shout_equipment_mult: Option<f32>,
    #[br(try)]
    pub unarmed_mult: Option<f32>,
    #[br(try)]
    pub staff_equipment_mult: Option<f32>,
    #[br(try)]
    pub avoid_threat_chance: Option<f32>,
}

impl TryFrom<CSGD> for GeneralStyle {
    type Error = Error;

    fn try_from(raw: CSGD) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeStyle {
    pub attack_staggered_mult: f32,
    pub power_attack_staggered_mult: f32,
    pub power_attack_blocking_mult: f32,
    pub bash_mult: f32,
    pub bash_recoiled_mult: f32,
    pub bash_attack_mult: f32,
    pub bash_power_mult: f32,
    #[br(try)]
    pub special_attack_mult: Option<f32>,
}

impl TryFrom<CSME> for MeleeStyle {
    type Error = Error;

    fn try_from(raw: CSME) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseRangeStyle {
    pub dueling_circle_mult: f32,
    pub dueling_fallback_mult: f32,
    #[br(try)]
    pub flank_distance: Option<f32>,
    #[br(try)]
    pub flanking_stalking_time: Option<f32>,
}

impl TryFrom<CSCR> for CloseRangeStyle {
    type Error = Error;

    fn try_from(raw: CSCR) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightStyle {
    pub hover_chance: f32,
    #[br(try)]
    pub dive_bomb_chance: Option<f32>,
    #[br(try)]
    pub ground_attack_chance: Option<f32>,
    #[br(try)]
    pub hover_time: Option<f32>,
    #[br(try)]
    pub ground_attack_time: Option<f32>,
    #[br(try)]
    pub perch_attack_chance: Option<f32>,
    #[br(try)]
    pub perch_attack_time: Option<f32>,
    #[br(try)]
    pub flying_attack_chance: Option<f32>,
}

impl TryFrom<CSFL> for FlightStyle {
    type Error = Error;

    fn try_from(raw: CSFL) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [CSTY] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStyle {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub general: GeneralStyle,
    pub melee: Option<MeleeStyle>,
    pub close_range: Option<CloseRangeStyle>,
    pub long_range: Option<f32>,
    pub flight: Option<FlightStyle>,
    pub flags: Option<u32>,
    pub unknown: Option<(f32, f32)>,
}

impl fmt::Display for CombatStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CombatStyle ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<CSTY> for CombatStyle {
    type Error = Error;

    fn try_from(raw: CSTY) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let general = CSGD::read(&mut cursor)?.try_into()?;
        let melee = CSME::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let close_range = CSCR::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let long_range = CSLR::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let flight = CSFL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let flags = DATA::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let unknown = CSMD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            general,
            melee,
            close_range,
            long_range,
            flight,
            flags,
            unknown,
        })
    }
}
