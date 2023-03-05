use crate::common::FormID;
use crate::error::Error;
use binrw::{binrw, helpers::until_eof, io::Cursor, BinRead, Endian};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"INAM")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct INAM {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryInto<FormID> for INAM {
    type Error = Error;

    fn try_into(self) -> Result<FormID, Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(FormID::read_le(&mut cursor)?)
    }
}

impl TryInto<Vec<FormID>> for INAM {
    type Error = Error;

    fn try_into(self) -> Result<Vec<FormID>, Error> {
        let mut cursor = Cursor::new(&self.data);
        Ok(until_eof(&mut cursor, Endian::Little, ())?)
    }
}
