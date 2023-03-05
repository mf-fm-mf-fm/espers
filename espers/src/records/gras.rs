use super::{get_cursor, Flags};
use crate::error::Error;
use crate::fields::{ObjectBounds, DATA, EDID, MODL, MODT, OBND};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"GRAS")]
pub struct GRAS {
    pub size: u32,
    pub flags: Flags,
    pub form_id: u32,
    pub timestamp: u16,
    pub version_control: u16,
    pub internal_version: u16,
    pub unknown: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grass {
    pub edid: String,
    pub bounds: ObjectBounds,
    pub model_filename: String,
    pub model_textures: Option<MODT>,
    pub data: DATA,
}

impl fmt::Display for Grass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grass ({})", self.edid)
    }
}

impl TryFrom<GRAS> for Grass {
    type Error = Error;

    fn try_from(raw: GRAS) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let model_filename = MODL::read(&mut cursor)?.try_into()?;
        let model_textures = MODT::read(&mut cursor).ok();
        let data = DATA::read(&mut cursor)?;

        Ok(Self {
            edid,
            bounds,
            model_filename,
            model_textures,
            data,
        })
    }
}
