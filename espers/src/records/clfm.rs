use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{CNAM, EDID, FNAM, FULL};
use binrw::{binrw, BinRead};
use rgb::RGBA8;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CLFM")]
pub struct CLFM {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub header: RecordHeader,
    pub edid: String,
    pub full_name: Option<String>,
    pub color: RGBA8,
    pub playable: u32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color ({})", self.edid)
    }
}

impl TryFrom<CLFM> for Color {
    type Error = Error;

    fn try_from(raw: CLFM) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let full_name = FULL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let color = CNAM::read(&mut cursor)?.try_into()?;
        let playable = FNAM::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            full_name,
            color,
            playable,
        })
    }
}
