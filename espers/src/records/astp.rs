use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{DATA, EDID, FCHT, FPRT, MCHT, MPRT};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ASTP")]
pub struct ASTP {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociationType {
    pub header: RecordHeader,
    pub edid: String,
    pub male_parent_label: String,
    pub female_parent_label: String,
    pub male_child_label: Option<String>,
    pub female_child_label: Option<String>,
    pub flags: u32,
}

impl fmt::Display for AssociationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AssociationType ({})", self.edid)
    }
}

impl TryFrom<ASTP> for AssociationType {
    type Error = Error;

    fn try_from(raw: ASTP) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let male_parent_label = MPRT::read(&mut cursor)?.try_into()?;
        let female_parent_label = FPRT::read(&mut cursor)?.try_into()?;
        let male_child_label = MCHT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let female_child_label = FCHT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let flags = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            male_parent_label,
            female_parent_label,
            male_child_label,
            female_child_label,
            flags,
        })
    }
}
