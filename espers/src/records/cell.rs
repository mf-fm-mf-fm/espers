use super::{get_cursor, Flags, RecordHeader};
use crate::common::LocalizedString;
use crate::error::Error;
use crate::fields::{EDID, FULL};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"CELL")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CELL {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub full_name: Option<LocalizedString>,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<CELL> for Cell {
    type Error = Error;

    fn try_from(raw: CELL) -> Result<Self, Self::Error> {
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

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
        })
    }
}
