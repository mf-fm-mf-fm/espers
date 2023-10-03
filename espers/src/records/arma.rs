use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{
    BodyTemplate, BodyTemplate2, Model, BOD2, BODT, DNAM, EDID, MO2S, MO2T, MO3S, MO3T, MO4S, MO4T,
    MO5S, MO5T, MOD2, MOD3, MOD4, MOD5, MODL, NAM0, NAM1, NAM2, NAM3, ONAM, RNAM, SNDD,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ARMA")]
pub struct ARMA {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorAddon {
    pub header: RecordHeader,
    pub edid: String,
    pub body_template: Option<BodyTemplate>,
    pub body_template_2: Option<BodyTemplate2>,
    pub primary_race: FormID,
    pub unknown: [u8; 12],
    pub male_model: Option<Model>,
    pub female_model: Option<Model>,
    pub male_3p_model: Option<Model>,
    pub female_3p_model: Option<Model>,
    pub base_male_texture: Option<FormID>,
    pub base_female_texture: Option<FormID>,
    pub base_male_1p_texture: Option<FormID>,
    pub base_female_1p_texture: Option<FormID>,
    pub races: Vec<FormID>,
    pub footstep_sound: Option<FormID>,
    pub art_object: Option<FormID>,
}

impl fmt::Display for ArmorAddon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArmorAddon ({})", self.edid)
    }
}

impl TryFrom<ARMA> for ArmorAddon {
    type Error = Error;

    fn try_from(raw: ARMA) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let body_template = BODT::read(&mut cursor)
            .ok()
            .map(|bodt| BodyTemplate::load(bodt, raw.header.internal_version))
            .transpose()?;
        let body_template_2 = BOD2::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let primary_race = RNAM::read(&mut cursor)?.try_into()?;
        let unknown = DNAM::read(&mut cursor)?.try_into()?;
        let male_model =
            Model::try_load::<MOD2, MO2T, MO2S>(&mut cursor, raw.header.internal_version)?;
        let female_model =
            Model::try_load::<MOD3, MO3T, MO3S>(&mut cursor, raw.header.internal_version)?;
        let male_3p_model =
            Model::try_load::<MOD4, MO4T, MO4S>(&mut cursor, raw.header.internal_version)?;
        let female_3p_model =
            Model::try_load::<MOD5, MO5T, MO5S>(&mut cursor, raw.header.internal_version)?;
        let base_male_texture = NAM0::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let base_female_texture = NAM1::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let base_male_1p_texture = NAM2::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let base_female_1p_texture = NAM3::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let mut races = Vec::new();

        while let Ok(e) = MODL::read(&mut cursor) {
            races.push(e.try_into()?);
        }

        let footstep_sound = SNDD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let art_object = ONAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            body_template,
            body_template_2,
            primary_race,
            unknown,
            male_model,
            female_model,
            male_3p_model,
            female_3p_model,
            base_male_texture,
            base_female_texture,
            base_male_1p_texture,
            base_female_1p_texture,
            races,
            footstep_sound,
            art_object,
        })
    }
}
