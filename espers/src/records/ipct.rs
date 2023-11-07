use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{
    DecalData, Model, DATA, DNAM, DODT, EDID, ENAM, MODL, MODS, MODT, NAM1, NAM2, SNAM,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"IPCT")]
pub struct IPCT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactDataData {
    pub effect_duration: f32,
    pub flags: u32,
    pub angle_threshold: f32,
    pub placement_radius: f32,
    pub sound_level: u32,
    pub unknown: u32,
}

impl TryFrom<DATA> for ImpactDataData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactData {
    pub header: RecordHeader,
    pub edid: String,
    pub model: Option<Model>,
    pub data: ImpactDataData,
    pub decal_data: Option<DecalData>,
    pub texture_set: Option<FormID>,
    pub secondary_texture_set: Option<FormID>,
    pub impact_sound_1: Option<FormID>,
    pub impact_sound_2: Option<FormID>,
    pub effect_hazard: Option<FormID>,
}

impl fmt::Display for ImpactData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImpactData ({})", self.edid)
    }
}

impl TryFrom<IPCT> for ImpactData {
    type Error = Error;

    fn try_from(raw: IPCT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let model = Model::try_load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;
        let data = DATA::read(&mut cursor)?.try_into()?;
        let decal_data = DODT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let texture_set = DNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let secondary_texture_set = ENAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let impact_sound_1 = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let impact_sound_2 = NAM1::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let effect_hazard = NAM2::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            model,
            data,
            decal_data,
            texture_set,
            secondary_texture_set,
            impact_sound_1,
            impact_sound_2,
            effect_hazard,
        })
    }
}
