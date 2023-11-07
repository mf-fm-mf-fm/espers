use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{DATA, EDID, MODT};
use binrw::{binrw, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [DEBR](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/DEBR) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"DEBR")]
pub struct DEBR {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionalData {
    pub percentage: u8,
    pub model_path: String,
    pub flags: u8,
}

impl TryFrom<DATA> for DirectionalData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let percentage = BinRead::read(&mut cursor)?;
        let model_path = NullString::read(&mut cursor)?.to_string();
        let flags = BinRead::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(Self {
            percentage,
            model_path,
            flags,
        })
    }
}

impl TryFrom<MODT> for Vec<(u32, u32, u32)> {
    type Error = Error;

    fn try_from(raw: MODT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let mut result = Vec::new();
        while let Ok(x) = BinRead::read_le(&mut cursor) {
            result.push(x);
        }
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [DEBR] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debris {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub data: Vec<(DirectionalData, Option<Vec<(u32, u32, u32)>>)>,
}

impl fmt::Display for Debris {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debris ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<DEBR> for Debris {
    type Error = Error;

    fn try_from(raw: DEBR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let mut data = Vec::new();
        while let Ok(d) = DATA::read(&mut cursor) {
            let modt = MODT::read(&mut cursor)
                .ok()
                .map(TryInto::try_into)
                .transpose()?;
            data.push((d.try_into()?, modt));
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            data,
        })
    }
}
