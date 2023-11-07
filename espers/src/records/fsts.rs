use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{DATA, EDID, XCNT};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [FSTS](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/FSTS) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"FSTS")]
pub struct FSTS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCount {
    pub walking: u32,
    pub running: u32,
    pub sprinting: u32,
    pub sneaking: u32,
    pub swimming: u32,
}

impl TryFrom<XCNT> for SetCount {
    type Error = Error;

    fn try_from(raw: XCNT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [FSTS] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootstepSet {
    pub header: RecordHeader,
    pub edid: String,
    pub counts: SetCount,
    pub sets: Vec<FormID>,
}

impl fmt::Display for FootstepSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FootstepSet ({})", self.edid)
    }
}

impl TryFrom<FSTS> for FootstepSet {
    type Error = Error;

    fn try_from(raw: FSTS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let counts = XCNT::read(&mut cursor)?.try_into()?;
        let sets = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            counts,
            sets,
        })
    }
}
