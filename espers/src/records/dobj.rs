use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::DNAM;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [DOBJ](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/DOBJ) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"DOBJ")]
pub struct DOBJ {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

impl TryFrom<DNAM> for Vec<([u8; 4], FormID)> {
    type Error = Error;

    fn try_from(raw: DNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(raw.data);
        let mut result = Vec::new();
        while let Ok(x) = BinRead::read_le(&mut cursor) {
            let y = FormID::read_le(&mut cursor)?;
            result.push((x, y))
        }

        check_done_reading(&mut cursor)?;

        Ok(result)
    }
}

/// Parsed [DOBJ] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultObjectManager {
    pub header: RecordHeader,
    pub items: Vec<([u8; 4], FormID)>,
}

impl fmt::Display for DefaultObjectManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DefaultObjectManager ({} items)", self.items.len())
    }
}

impl TryFrom<DOBJ> for DefaultObjectManager {
    type Error = Error;

    fn try_from(raw: DOBJ) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let items: Vec<_> = DNAM::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            items,
        })
    }
}
