use super::{get_cursor, Flags, RecordHeader};
use crate::common::{FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    Condition, EffectItem, ObjectBounds, CTDA, EDID, EFID, EFIT, ENIT, FULL, OBND,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"ENCH")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ENCH {
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
pub struct EnchantedItem {
    pub cost: u32,
    pub flags: u32,
    pub cast_type: u32,
    pub amount: u32,
    pub delivery: u32,
    pub kind: u32,
    pub charge_time: f32,
    pub base_enchantment: u32,
    #[br(try)]
    pub worn_restrictions: Option<u32>,
}

impl TryFrom<ENIT> for EnchantedItem {
    type Error = Error;

    fn try_from(raw: ENIT) -> Result<Self, Self::Error> {
        Ok(Self::read(&mut Cursor::new(&raw.data))?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enchantment {
    pub header: RecordHeader,
    pub edid: String,
    pub obnd: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub item: EnchantedItem,
    pub effects: Vec<(FormID, EffectItem, Vec<Condition>)>,
}

impl fmt::Display for Enchantment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enchantment ({})", self.edid)
    }
}

impl TryFrom<ENCH> for Enchantment {
    type Error = Error;

    fn try_from(raw: ENCH) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let obnd = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let item = ENIT::read(&mut cursor)?.try_into()?;

        let mut effects = Vec::new();

        while let Ok(efid) = EFID::read(&mut cursor) {
            let efit = EFIT::read(&mut cursor)?.try_into()?;
            let mut ctdas = Vec::new();

            while let Ok(ctda) = CTDA::read(&mut cursor) {
                ctdas.push(ctda.try_into()?);
            }

            effects.push((efid.try_into()?, efit, ctdas));
        }

        Ok(Self {
            header: raw.header,
            edid,
            obnd,
            full_name,
            item,
            effects,
        })
    }
}
