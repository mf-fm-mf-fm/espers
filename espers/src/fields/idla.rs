use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use binrw::{binrw, io::Cursor, until_eof, Endian};
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"IDLA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IDLA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<IDLA> for Vec<FormID> {
    type Error = Error;

    fn try_from(raw: IDLA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = until_eof(&mut cursor, Endian::Little, ())?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}
