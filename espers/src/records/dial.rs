use super::{get_cursor, Flags, RecordHeader};
use crate::common::LocalizedString;
use crate::error::Error;
use crate::fields::{EDID, FULL, PNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTopic {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub full_name: Option<LocalizedString>,
    pub priority: f32,
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

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            priority,
        })
    }
}
