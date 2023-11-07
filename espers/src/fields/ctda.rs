use super::{CIS1, CIS2, CITC};
use crate::common::check_done_reading;
use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"CTDA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CTDA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub operator: u8,
    pub unknown1: [u8; 3],
    pub comparison_value: f32,
    pub function_index: u16,
    pub padding: u16,
    pub param1: i32,
    pub param2: i32,
    // pub param1: u16,
    // pub param2: [u8; 2],
    // pub param3: u32,
    pub run_on_type: u32,
    pub reference: u32,
    pub unknown2: i32,
}

impl TryFrom<CTDA> for Condition {
    type Error = Error;

    fn try_from(raw: CTDA) -> Result<Condition, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EffectCondition {
    pub condition: Condition,
    pub condition_item_count: Option<u32>,
    pub param1_override: Option<String>,
    pub param2_override: Option<String>,
}

impl EffectCondition {
    pub fn load(cursor: &mut Cursor<&Vec<u8>>) -> Result<Self, Error> {
        let condition_item_count = CITC::read(cursor).ok().map(TryInto::try_into).transpose()?;
        let condition = CTDA::read(cursor)?.try_into()?;
        let condition_item_count = match condition_item_count {
            citc @ Some(_) => citc,
            None => CITC::read(cursor).ok().map(TryInto::try_into).transpose()?,
        };
        let param1_override = CIS1::read(cursor).ok().map(TryInto::try_into).transpose()?;
        let param2_override = CIS2::read(cursor).ok().map(TryInto::try_into).transpose()?;

        Ok(Self {
            condition,
            condition_item_count,
            param1_override,
            param2_override,
        })
    }

    pub fn load_multiple(cursor: &mut Cursor<&Vec<u8>>) -> Result<Vec<Self>, Error> {
        let mut items = Vec::new();
        while let Ok(m) = Self::load(cursor) {
            items.push(m);
        }
        Ok(items)
    }
}
