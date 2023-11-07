use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    CrimeGold, EffectCondition, CRGR, CRVA, DATA, EDID, FNAM, FULL, JAIL, JOUT, MNAM, PLCN, PLVD,
    RNAM, STOL, VENC, VEND, VENV, WAIT, XNAM,
};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [FACT](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/FACT) record
#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"FACT")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FACT {
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
pub struct Relations {
    pub faction: FormID,
    pub m: i32,
    pub combat: i32,
}

impl TryFrom<XNAM> for Relations {
    type Error = Error;

    fn try_from(raw: XNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct FactionFlags: u32 {
        const HIDDEN_FROM_PC = 0x1;
        const SPECIAL_COMBAT = 0x2;
        const TRACK_CRIME = 0x40;
        const IGNORE_MURDER = 0x80;
        const IGNORE_ASSAULT = 0x100;
        const IGNORE_STEALING = 0x200;
        const IGNORE_TRESPASS = 0x400;
        const NO_MEMBER_REPORTING = 0x800;
        const CRIME_GOLD = 0x1000;
        const IGNORE_PICKPOCKET = 0x2000;
        const VENDOR = 0x4000;
        const CAN_BE_OWNER = 0x8000;
        const IGNORE_WEREWOLF = 0x10000;
    }
}

impl TryFrom<DATA> for FactionFlags {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rank {
    pub id: u32,
    pub male_title: Option<LocalizedString>,
    pub female_title: Option<LocalizedString>,
}

impl Rank {
    pub fn load(cursor: &mut Cursor<&Vec<u8>>, localized: bool) -> Result<Self, Error> {
        let id = RNAM::read(cursor)?.try_into()?;
        let male_title = match (MNAM::read(cursor), localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let female_title = match (FNAM::read(cursor), localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };

        Ok(Self {
            id,
            male_title,
            female_title,
        })
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorInfo {
    pub start_hour: u16,
    pub end_hour: u16,
    pub radius: u32,
    pub fencer: u8,
    pub merchandise_invert: u8,
    pub unused: u16,
}

impl TryFrom<VENV> for VendorInfo {
    type Error = Error;

    fn try_from(raw: VENV) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorPlace {
    pub kind: u32,
    pub form_id: FormID,
    pub unused: u32,
}

impl TryFrom<PLVD> for VendorPlace {
    type Error = Error;

    fn try_from(raw: PLVD) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [FACT] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<LocalizedString>,
    pub relations: Vec<Relations>,
    pub flags: FactionFlags,
    pub jail: Option<FormID>,
    pub follower_wait_marker: Option<FormID>,
    pub evidence_chest: Option<FormID>,
    pub belongings_chest: Option<FormID>,
    pub crime_group: Option<FormID>,
    pub jail_outfit: Option<FormID>,
    pub crime_gold: Option<CrimeGold>,
    pub ranks: Vec<Rank>,
    pub merchandise: Option<FormID>,
    pub chest: Option<FormID>,
    pub vendor: Option<VendorInfo>,
    pub place: Option<VendorPlace>,
    pub conditions: Vec<EffectCondition>,
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Faction ({})", self.edid)
    }
}

impl TryFrom<FACT> for Faction {
    type Error = Error;

    fn try_from(raw: FACT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let mut relations = Vec::new();
        while let Ok(x) = XNAM::read(&mut cursor) {
            relations.push(x.try_into()?);
        }

        let flags = DATA::read(&mut cursor)?.try_into()?;
        let jail = JAIL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let follower_wait_marker = WAIT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let evidence_chest = STOL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let belongings_chest = PLCN::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let crime_group = CRGR::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let jail_outfit = JOUT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let crime_gold = CRVA::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut ranks = Vec::new();
        while let Ok(r) = Rank::load(&mut cursor, raw.localized) {
            ranks.push(r);
        }
        let merchandise = VEND::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let chest = VENC::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let vendor = VENV::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let place = PLVD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut conditions = Vec::new();
        while let Ok(c) = EffectCondition::load(&mut cursor) {
            conditions.push(c);
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            relations,
            flags,
            jail,
            follower_wait_marker,
            evidence_chest,
            belongings_chest,
            crime_group,
            jail_outfit,
            crime_gold,
            ranks,
            merchandise,
            chest,
            vendor,
            place,
            conditions,
        })
    }
}
