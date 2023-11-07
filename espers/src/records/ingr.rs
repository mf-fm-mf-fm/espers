use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    Condition, EffectItem, Model, ObjectBounds, ScriptList, CTDA, DATA, EDID, EFID, EFIT, ENIT,
    FULL, ICON, KSIZ, KWDA, MODL, MODS, MODT, OBND, VMAD, YNAM, ZNAM,
};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"INGR")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct INGR {
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
pub struct IngredientData {
    pub value: u32,
    pub weight: f32,
}

impl TryFrom<DATA> for IngredientData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct EffectDataFlags: u32 {
        const MANUAL_CALC = 0x001;
        const FOOD = 0x002;
        const REFERENCES_PERSIST = 0x100;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectData {
    pub value: u32,
    pub flags: EffectDataFlags,
}

impl TryFrom<ENIT> for EffectData {
    type Error = Error;

    fn try_from(raw: ENIT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: LocalizedString,
    pub keywords: Vec<FormID>,
    pub model: Model,
    pub inventory_image: Option<String>,
    pub pickup_sound: Option<FormID>,
    pub drop_sound: Option<FormID>,
    pub data: IngredientData,
    pub effect_data: EffectData,
    pub effects: Vec<(FormID, EffectItem, Vec<Condition>)>,
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ingredient ({})", self.edid)
    }
}

impl TryFrom<INGR> for Ingredient {
    type Error = Error;

    fn try_from(raw: INGR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor)?, raw.localized) {
            (f, true) => LocalizedString::Localized(f.try_into()?),
            (z, false) => LocalizedString::ZString(z.try_into()?),
        };
        let keyword_count: Option<u32> = KSIZ::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut keywords = Vec::new();

        if let Some(kc) = keyword_count {
            for _ in 0..kc {
                // It's actually only up to keyword count
                if let Ok(kwda) = KWDA::read(&mut cursor) {
                    keywords.push(FormID::read_le(&mut Cursor::new(kwda.data)).unwrap());
                }
            }
        }
        let model = Model::load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;
        let inventory_image = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let pickup_sound = YNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let drop_sound = ZNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let data = DATA::read(&mut cursor)?.try_into()?;
        let effect_data = ENIT::read(&mut cursor)?.try_into()?;

        let mut effects = Vec::new();
        while let Ok(efid) = EFID::read(&mut cursor) {
            let efit = EFIT::read(&mut cursor)?.try_into()?;
            let mut ctdas = Vec::new();

            while let Ok(ctda) = CTDA::read(&mut cursor) {
                ctdas.push(ctda.try_into()?);
            }

            effects.push((efid.try_into()?, efit, ctdas));
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            keywords,
            model,
            inventory_image,
            pickup_sound,
            drop_sound,
            data,
            effect_data,
            effects,
        })
    }
}
