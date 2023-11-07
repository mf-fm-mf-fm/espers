use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"CSFL")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CSFL {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}
