use super::{ModelTextures, ReadTextures, Textures, Unknown4};
use crate::{common::check_done_reading, error::Error};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

macro_rules! impl_model {
    ($modt:ident, $modt_lit:literal) => {
        #[binrw]
        #[brw(little, magic = $modt_lit)]
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $modt {
            pub size: u16,

            #[br(count = size)]
            pub data: Vec<u8>,
        }

        impl ReadTextures for $modt {
            fn read_textures(
                cursor: &mut Cursor<&Vec<u8>>,
                version: u16,
            ) -> Result<Textures, Error> {
                Ok((Self::read_le(cursor)?, version).try_into()?)
            }
            fn try_read_textures(
                cursor: &mut Cursor<&Vec<u8>>,
                version: u16,
            ) -> Result<Option<Textures>, Error> {
                match Self::read_le(cursor) {
                    Ok(m) => Ok(Some((m, version).try_into()?)),
                    Err(_) => return Ok(None),
                }
            }
        }

        impl TryFrom<($modt, u16)> for Textures {
            type Error = Error;

            fn try_from((modt, version): ($modt, u16)) -> Result<Self, Error> {
                Ok(match version {
                    0..=37 => Err(Error::UnknownVersion(format!("{:?}", modt), version))?,
                    38..=39 => Self::NoHeader(modt.try_into()?),
                    40.. => Self::Header(modt.try_into()?),
                })
            }
        }

        impl TryInto<ModelTextures> for $modt {
            type Error = Error;

            fn try_into(self) -> Result<ModelTextures, Error> {
                Ok(ModelTextures::read(&mut Cursor::new(&self.data))?)
            }
        }

        impl TryFrom<$modt> for Vec<Unknown4> {
            type Error = Error;

            fn try_from(raw: $modt) -> Result<Self, Self::Error> {
                let mut cursor = Cursor::new(&raw.data);
                let mut parsed = Vec::new();
                while let Ok(result) = Unknown4::read_le(&mut cursor) {
                    parsed.push(result);
                }
                check_done_reading(&mut cursor)?;
                Ok(parsed)
            }
        }
    };
}

impl_model!(MODT, b"MODT");
impl_model!(MO2T, b"MO2T");
impl_model!(MO3T, b"MO3T");
impl_model!(MO4T, b"MO4T");
impl_model!(MO5T, b"MO5T");
