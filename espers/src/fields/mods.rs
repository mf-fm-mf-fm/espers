use super::model::{AlternateTexture, AlternateTextures, ReadAlternateTextures};
use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

macro_rules! impl_model {
    ($mods:ident, $mods_lit:literal) => {
        #[binrw]
        #[brw(little, magic = $mods_lit)]
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $mods {
            pub size: u16,

            #[br(count = size)]
            pub data: Vec<u8>,
        }

        impl ReadAlternateTextures for $mods {
            fn read_alt_textures(
                cursor: &mut Cursor<&Vec<u8>>,
            ) -> Result<AlternateTextures, Error> {
                Ok(Self::read_le(cursor)?.try_into()?)
            }
            fn try_read_alt_textures(
                cursor: &mut Cursor<&Vec<u8>>,
            ) -> Result<Option<AlternateTextures>, Error> {
                match Self::read_le(cursor) {
                    Ok(m) => Ok(Some(m.try_into()?)),
                    Err(_) => return Ok(None),
                }
            }
        }

        impl TryInto<AlternateTextures> for $mods {
            type Error = Error;

            fn try_into(self) -> Result<AlternateTextures, Error> {
                Ok(AlternateTextures::read(&mut Cursor::new(&self.data))?)
            }
        }

        impl TryFrom<$mods> for Vec<AlternateTexture> {
            type Error = Error;

            fn try_from(raw: $mods) -> Result<Self, Self::Error> {
                let mut cursor = Cursor::new(&raw.data);
                let mut parsed = Vec::new();
                while let Ok(result) = AlternateTexture::read_le(&mut cursor) {
                    parsed.push(result);
                }
                check_done_reading(&mut cursor)?;
                Ok(parsed)
            }
        }
    };
}

impl_model!(MODS, b"MODS");
impl_model!(MO2S, b"MO2S");
impl_model!(MO3S, b"MO3S");
impl_model!(MO4S, b"MO4S");
impl_model!(MO5S, b"MO5S");
