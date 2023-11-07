use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{EffectCondition, ANAM, DATA, EDID, SNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [CPTH](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/CPTH) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CPTH")]
pub struct CPTH {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

/// Parsed [CPTH] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraPath {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub conditions: Vec<EffectCondition>,
    pub paths: (FormID, FormID),
    pub flags: u8,
    pub cameras: Vec<FormID>,
}

impl fmt::Display for CameraPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CameraPath ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<CPTH> for CameraPath {
    type Error = Error;

    fn try_from(raw: CPTH) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let mut conditions = Vec::new();
        while let Ok(c) = EffectCondition::load(&mut cursor) {
            conditions.push(c);
        }
        let paths = ANAM::read(&mut cursor)?.try_into()?;
        let flags = DATA::read(&mut cursor)?.try_into()?;
        let mut cameras = Vec::new();
        while let Ok(c) = SNAM::read(&mut cursor) {
            cameras.push(c.try_into()?);
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            conditions,
            paths,
            flags,
            cameras,
        })
    }
}
