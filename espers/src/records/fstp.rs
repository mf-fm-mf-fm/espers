use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{ANAM, DATA, EDID};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [FSTP](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/FSTP) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"FSTP")]
pub struct FSTP {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

/// Parsed [FSTP] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footstep {
    pub header: RecordHeader,
    pub edid: String,
    pub impact_data: FormID,
    pub action_name: String,
}

impl fmt::Display for Footstep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Footstep ({})", self.edid)
    }
}

impl TryFrom<FSTP> for Footstep {
    type Error = Error;

    fn try_from(raw: FSTP) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let impact_data = DATA::read(&mut cursor)?.try_into()?;
        let action_name = ANAM::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            impact_data,
            action_name,
        })
    }
}
