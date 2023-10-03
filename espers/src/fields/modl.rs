use super::model::ReadModel;
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};

macro_rules! impl_model {
    ($modl:ident, $modl_lit:literal) => {
        #[binrw]
        #[brw(little, magic = $modl_lit)]
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $modl {
            pub size: u16,

            #[br(count = size)]
            pub data: Vec<u8>,
        }

        impl TryFrom<$modl> for String {
            type Error = Error;

            fn try_from(raw: $modl) -> Result<Self, Self::Error> {
                let mut cursor = Cursor::new(&raw.data);
                let result = NullString::read_le(&mut cursor)?.to_string();
                check_done_reading(&mut cursor)?;
                Ok(result)
            }
        }

        impl TryFrom<$modl> for FormID {
            type Error = Error;

            fn try_from(raw: $modl) -> Result<Self, Self::Error> {
                let mut cursor = Cursor::new(&raw.data);
                let result = Self::read_le(&mut cursor)?;
                check_done_reading(&mut cursor)?;
                Ok(result)
            }
        }

        impl ReadModel for $modl {
            fn read_model(cursor: &mut Cursor<&Vec<u8>>) -> Result<String, Error> {
                Ok(Self::read_le(cursor)?.try_into()?)
            }
            fn try_read_model(cursor: &mut Cursor<&Vec<u8>>) -> Result<Option<String>, Error> {
                match Self::read_le(cursor) {
                    Ok(m) => Ok(Some(m.try_into()?)),
                    Err(_) => {
                        return Ok(None);
                    }
                }
            }
        }
    };
}

impl_model!(MODL, b"MODL");
impl_model!(MOD2, b"MOD2");
impl_model!(MOD3, b"MOD3");
impl_model!(MOD4, b"MOD4");
impl_model!(MOD5, b"MOD5");
