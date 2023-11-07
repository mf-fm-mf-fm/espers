use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{BNAM, DATA, EDID, FULL, PNAM, QNAM, SNAM, TIFC};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [DIAL](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/DIAL) record
#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"DIAL")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIAL {
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
pub struct DialogTopicData {
    pub unknown: u8,
    pub tab: u8,
    pub subtype: u8,
    pub unused: u8,
}

impl TryFrom<DATA> for DialogTopicData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

impl TryFrom<SNAM> for [u8; 4] {
    type Error = Error;

    fn try_from(raw: SNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [DIAL] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTopic {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub full_name: Option<LocalizedString>,
    pub priority: f32,
    pub owning_branch: Option<FormID>,
    pub owning_quest: FormID,
    pub data: DialogTopicData,
    pub subtype: [u8; 4],
    pub info_count: u32,
}

impl fmt::Display for DialogueTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DialogueTopic ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<DIAL> for DialogueTopic {
    type Error = Error;

    fn try_from(raw: DIAL) -> Result<Self, Self::Error> {
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
        let priority = PNAM::read(&mut cursor)?.try_into()?;
        let owning_branch = BNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let owning_quest = QNAM::read(&mut cursor)?.try_into()?;
        let data = DATA::read(&mut cursor)?.try_into()?;
        let subtype = SNAM::read(&mut cursor)?.try_into()?;
        let info_count = TIFC::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            priority,
            owning_branch,
            owning_quest,
            data,
            subtype,
            info_count,
        })
    }
}
