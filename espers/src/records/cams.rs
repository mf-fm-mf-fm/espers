use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{Model, DATA, EDID, MNAM, MODL, MODS, MODT};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CAMS")]
pub struct CAMS {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraShot {
    pub header: RecordHeader,
    pub edid: String,
    pub model: Option<Model>,
    pub data: Vec<u8>,
    pub effect: Option<FormID>,
}

impl fmt::Display for CameraShot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CameraShot ({})", self.edid)
    }
}

impl TryFrom<CAMS> for CameraShot {
    type Error = Error;

    fn try_from(raw: CAMS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let model = Model::try_load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;
        let data = DATA::read(&mut cursor)?.try_into()?;
        let effect = MNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            model,
            data,
            effect,
        })
    }
}
