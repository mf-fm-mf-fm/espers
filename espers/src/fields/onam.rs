use crate::common::FormID;
use crate::error::Error;
use binrw::{binrw, helpers::until_eof, BinWrite, Endian};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"ONAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ONAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<Vec<FormID>> for ONAM {
    type Error = Error;

    fn try_into(self) -> Result<Vec<FormID>, Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(until_eof(&mut cursor, Endian::Little, ())?)
    }
}

impl TryFrom<Vec<FormID>> for ONAM {
    type Error = Error;

    fn try_from(obj: Vec<FormID>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(Vec::new());
        for fid in obj {
            fid.0.write_le(&mut cursor)?;
        }
        let data = cursor.into_inner();

        Ok(Self {
            size: data.len() as u16,
            data,
        })
    }
}
