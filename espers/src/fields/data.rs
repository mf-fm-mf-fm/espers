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
