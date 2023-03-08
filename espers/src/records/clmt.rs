use super::{get_cursor, Flags, RecordHeader};
use crate::error::Error;
use crate::fields::{ModelTextures, SunAndMoons, EDID, FNAM, GNAM, MODL, MODT, TNAM, WLST};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"CLMT")]
pub struct CLMT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Climate {
    pub header: RecordHeader,
    pub edid: String,
    pub wlst: WLST,
    pub sun_texture: Option<String>,
    pub glare_texture: Option<String>,
    pub model_filename: Option<String>,
    pub model_textures: Option<ModelTextures>,
    pub sun_and_moons: SunAndMoons,
}

impl fmt::Display for Climate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Climate ({})", self.edid)
    }
}

impl TryFrom<CLMT> for Climate {
    type Error = Error;

    fn try_from(raw: CLMT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let wlst = WLST::read(&mut cursor)?;
        let sun_texture = FNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let glare_texture = GNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_filename = MODL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let sun_and_moons = TNAM::read(&mut cursor)?.try_into()?;

        Ok(Self {
            header: raw.header,
            edid,
            wlst,
            sun_texture,
            glare_texture,
            model_filename,
            model_textures,
            sun_and_moons,
        })
    }
}
