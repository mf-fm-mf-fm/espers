use binrw::binrw;
use serde_derive::{Deserialize, Serialize};

#[binrw]
#[brw(little, magic = b"XCLL")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XCLL {
    pub size: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}
