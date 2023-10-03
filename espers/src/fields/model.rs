use crate::{
    common::{FormID, WString32},
    error::Error,
};
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

pub trait ReadModel {
    fn read_model(cursor: &mut Cursor<&Vec<u8>>) -> Result<String, Error>;
    fn try_read_model(cursor: &mut Cursor<&Vec<u8>>) -> Result<Option<String>, Error>;
}

pub trait ReadTextures {
    fn read_textures(cursor: &mut Cursor<&Vec<u8>>, version: u16) -> Result<Textures, Error>;
    fn try_read_textures(
        cursor: &mut Cursor<&Vec<u8>>,
        version: u16,
    ) -> Result<Option<Textures>, Error>;
}

pub trait ReadAlternateTextures {
    fn read_alt_textures(cursor: &mut Cursor<&Vec<u8>>) -> Result<AlternateTextures, Error>;
    fn try_read_alt_textures(
        cursor: &mut Cursor<&Vec<u8>>,
    ) -> Result<Option<AlternateTextures>, Error>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Textures {
    Header(ModelTextures),
    NoHeader(Vec<Unknown4>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Model {
    pub model: String,
    pub textures: Option<Textures>,
    pub alternate_textures: Option<AlternateTextures>,
}

impl Model {
    pub fn load<M, D, AT>(cursor: &mut Cursor<&Vec<u8>>, version: u16) -> Result<Self, Error>
    where
        M: ReadModel,
        D: ReadTextures,
        AT: ReadAlternateTextures,
    {
        Ok(Self {
            model: M::read_model(cursor)?,
            textures: D::try_read_textures(cursor, version)?,
            alternate_textures: AT::try_read_alt_textures(cursor)?,
        })
    }

    pub fn try_load<M, D, AT>(
        cursor: &mut Cursor<&Vec<u8>>,
        version: u16,
    ) -> Result<Option<Self>, Error>
    where
        M: ReadModel,
        D: ReadTextures,
        AT: ReadAlternateTextures,
    {
        let model = M::try_read_model(cursor)?;
        let textures = D::try_read_textures(cursor, version)?;
        let alternate_textures = AT::try_read_alt_textures(cursor)?;

        match model {
            Some(m) => Ok(Some(Self {
                model: m,
                textures,
                alternate_textures,
            })),
            None => Ok(None),
        }
    }
}
#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlternateTexture {
    pub object_name: WString32,
    pub texture_set: FormID,
    pub threed_index: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlternateTextures {
    pub count: u32,

    #[br(count = count)]
    pub textures: Vec<AlternateTexture>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Unknown4 {
    pub unknown1: u32,
    pub unknown2: [u8; 4],
    pub unknown3: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelTextures {
    pub count: u32,
    pub unknown4_count: u32,
    pub unknown5_count: u32,

    #[br(count = unknown4_count)]
    pub unknown4s: Vec<Unknown4>,

    #[br(count = unknown5_count)]
    pub unknown5: Vec<u32>,
}
