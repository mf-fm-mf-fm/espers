use crate::common::check_done_reading;
use crate::common::FormID;
use crate::error::Error;
use binrw::{binrw, BinRead, NullString};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

macro_rules! impl_name {
    ($name:ident, $name_lit:literal) => {
        #[binrw]
        #[brw(little, magic = $name_lit)]
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $name {
            pub size: u16,

            #[br(count = size)]
            pub data: Vec<u8>,
        }

        impl TryFrom<$name> for FormID {
            type Error = Error;

            fn try_from(raw: $name) -> Result<Self, Error> {
                let mut cursor = Cursor::new(&raw.data);
                let result = Self::read_le(&mut cursor)?;
                check_done_reading(&mut cursor)?;
                Ok(result)
            }
        }

        impl TryFrom<$name> for String {
            type Error = Error;

            fn try_from(raw: $name) -> Result<Self, Self::Error> {
                let mut cursor = Cursor::new(&raw.data);
                let result = NullString::read_le(&mut cursor)?.to_string();
                check_done_reading(&mut cursor)?;
                Ok(result)
            }
        }
    };
}

impl_name!(NAME, b"NAME");
impl_name!(NAM0, b"NAM0");
impl_name!(NAM1, b"NAM1");
impl_name!(NAM2, b"NAM2");
impl_name!(NAM3, b"NAM3");
impl_name!(NAM4, b"NAM4");
impl_name!(NAM5, b"NAM5");
impl_name!(NAM6, b"NAM6");
impl_name!(NAM7, b"NAM7");
impl_name!(NAM8, b"NAM8");
impl_name!(NAM9, b"NAM9");

impl TryFrom<NAM5> for Vec<u8> {
    type Error = Error;

    fn try_from(raw: NAM5) -> Result<Self, Self::Error> {
        Ok(raw.data)
    }
}
