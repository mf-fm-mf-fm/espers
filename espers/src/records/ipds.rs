use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{EDID, PNAM};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"IPDS")]
pub struct IPDS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactDataItem {
    pub material_type: FormID,
    pub impact_data: FormID,
}

impl TryFrom<PNAM> for ImpactDataItem {
    type Error = Error;

    fn try_from(raw: PNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactDataSet {
    pub header: RecordHeader,
    pub edid: String,
    pub items: Vec<ImpactDataItem>,
}

impl fmt::Display for ImpactDataSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImpactDataSet ({})", self.edid)
    }
}

impl TryFrom<IPDS> for ImpactDataSet {
    type Error = Error;

    fn try_from(raw: IPDS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let mut items = Vec::new();
        while let Ok(i) = PNAM::read(&mut cursor) {
            items.push(i.try_into()?)
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            items,
        })
    }
}
