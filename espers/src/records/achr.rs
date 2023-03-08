use super::{get_cursor, Flags, RecordHeader};
use crate::common::FormID;
use crate::error::Error;
use crate::fields::{ScriptList, EDID, NAME, VMAD, XEZN};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"ACHR")]
pub struct ACHR {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRef {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub scripts: Option<ScriptList>,
    pub name: FormID,
    pub encounter_zone: Option<FormID>,
}

impl fmt::Display for ActorRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorRef ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<ACHR> for ActorRef {
    type Error = Error;

    fn try_from(raw: ACHR) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(|c| c.try_into())
            .transpose()?;

        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let name = NAME::read(&mut cursor)?.try_into()?;
        let encounter_zone = XEZN::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            name,
            encounter_zone,
        })
    }
}
