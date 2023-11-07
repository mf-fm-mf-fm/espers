use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XCNT")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XCNT {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}
