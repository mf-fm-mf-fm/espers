use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"CSME")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CSME {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}
