use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::EDID;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"EFSH")]
pub struct EFSH {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectShader {
    pub header: RecordHeader,
    pub edid: Option<String>,
}

impl fmt::Display for EffectShader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EffectShader ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<EFSH> for EffectShader {
    type Error = Error;

    fn try_from(raw: EFSH) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        Ok(Self {
            header: raw.header,
            edid,
        })
    }
}
