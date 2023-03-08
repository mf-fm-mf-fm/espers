use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{DATA, EDID};
use binrw::{binrw, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"GMST")]
pub struct GMST {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Bool(u32),
    Int(u32),
    Float(f32),
    Str(String),
    Unknown([u8; 4]),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSetting {
    pub header: RecordHeader,
    pub edid: String,
    pub value: Value,
}

impl fmt::Display for GameSetting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameSetting ({})", self.edid)
    }
}

impl TryFrom<GMST> for GameSetting {
    type Error = Error;

    fn try_from(raw: GMST) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid: String = EDID::read(&mut cursor)?.try_into()?;
        let data = DATA::read(&mut cursor)?;
        let mut data_cursor = Cursor::new(&data.data);
        let value = match &edid.chars().next() {
            Some('b') => Value::Bool(u32::read_le(&mut data_cursor)?),
            Some('i') => Value::Int(u32::read_le(&mut data_cursor)?),
            Some('f') => Value::Float(f32::read_le(&mut data_cursor)?),
            Some('s') => Value::Str(NullString::read_le(&mut data_cursor)?.to_string()),
            _ => Value::Unknown(BinRead::read_le(&mut data_cursor)?),
        };

        Ok(Self {
            header: raw.header,
            edid,
            value,
        })
    }
}
