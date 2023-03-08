use crate::error::Error;
use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"DATA")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DATA {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl TryFrom<Vec<u8>> for DATA {
    type Error = Error;

    fn try_from(obj: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            size: obj.len() as u16,
            data: obj,
        })
    }
}
