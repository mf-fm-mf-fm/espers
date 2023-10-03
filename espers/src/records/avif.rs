use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    ANAM, AVSK, CNAM, DESC, EDID, FNAM, FULL, HNAM, INAM, PNAM, SNAM, VNAM, XNAM, YNAM,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"AVIF")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AVIF {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerkSection {
    pub perk: FormID,
    pub flag: u32,
    pub xcoord: u32,
    pub ycoord: u32,
    pub horizontal_position: f32,
    pub vertical_position: f32,
    pub skill: FormID,
    pub lines: Vec<u32>,
    pub index_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorValue {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<LocalizedString>,
    pub description: LocalizedString,
    pub abbreviation: Option<String>,
    pub av_data: Option<[f32; 4]>,
    pub data: Vec<u32>,
    pub perk_sections: Vec<PerkSection>,
}

impl fmt::Display for ActorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorValue ({})", self.edid)
    }
}

impl TryFrom<AVIF> for ActorValue {
    type Error = Error;

    fn try_from(raw: AVIF) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let description = if raw.localized {
            LocalizedString::Localized(DESC::read(&mut cursor)?.try_into()?)
        } else {
            LocalizedString::ZString(DESC::read(&mut cursor)?.try_into()?)
        };
        let abbreviation = ANAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let mut data = Vec::new();
        while let Ok(e) = CNAM::read(&mut cursor) {
            data.push(e.try_into()?);
        }

        let av_data = AVSK::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let mut perk_sections = Vec::new();
        while let Ok(p) = PNAM::read(&mut cursor) {
            let perk = p.try_into()?;
            let flag = FNAM::read(&mut cursor)?.try_into()?;
            let xcoord = XNAM::read(&mut cursor)?.try_into()?;
            let ycoord = YNAM::read(&mut cursor)?.try_into()?;
            let horizontal_position = HNAM::read(&mut cursor)?.try_into()?;
            let vertical_position = VNAM::read(&mut cursor)?.try_into()?;
            let skill = SNAM::read(&mut cursor)?.try_into()?;
            let mut lines = Vec::new();
            while let Ok(l) = CNAM::read(&mut cursor) {
                lines.push(l.try_into()?);
            }
            let index_number = INAM::read(&mut cursor)?.try_into()?;

            perk_sections.push(PerkSection {
                perk,
                flag,
                xcoord,
                ycoord,
                horizontal_position,
                vertical_position,
                skill,
                lines,
                index_number,
            })
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            description,
            abbreviation,
            data,
            av_data,
            perk_sections,
        })
    }
}
