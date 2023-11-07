use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{EffectCondition, BNAM, CNAM, CNTO, COCT, COED, EDID, NAM1};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [COBJ](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/COBJ) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"COBJ")]
pub struct COBJ {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

/// Parsed [COBJ] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructibleObj {
    pub header: RecordHeader,
    pub edid: String,
    pub object_count: Option<u32>,
    pub objects: Vec<(FormID, u32)>,
    pub unknown: Option<COED>,
    pub conditions: Vec<EffectCondition>,
    pub output: Option<FormID>,
    pub bench: FormID,
    pub quantity: u16,
}

impl fmt::Display for ConstructibleObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstructibleObj ({})", self.edid)
    }
}

impl TryFrom<COBJ> for ConstructibleObj {
    type Error = Error;

    fn try_from(raw: COBJ) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let object_count = COCT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut objects = Vec::new();
        while let Ok(obj) = CNTO::read(&mut cursor) {
            objects.push(obj.try_into()?);
        }
        let unknown = COED::read(&mut cursor).ok();
        let conditions = EffectCondition::load_multiple(&mut cursor)?;
        let output = CNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bench = BNAM::read(&mut cursor)?.try_into()?;
        let quantity = NAM1::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            object_count,
            objects,
            unknown,
            conditions,
            output,
            bench,
            quantity,
        })
    }
}
