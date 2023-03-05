use crate::error::Error;
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::io::Cursor;

#[binrw]
#[brw(little, magic = b"VMAD")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VMAD {
    pub size: u16,
    #[br(count = size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    #[brw(magic = 1u8)]
    Type1 { status: u8, data: [u32; 2] },
    #[brw(magic = 3u8)]
    Type2 { status: u8, data: u32 },
    #[brw(magic = 5u8)]
    Type3 { status: u8, data: u8 },
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawProperty {
    pub name_size: u16,
    #[br(count = name_size)]
    pub name: Vec<u8>,

    pub kind: PropertyType,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawScript {
    pub name_size: u16,
    #[br(count = name_size)]
    pub name: Vec<u8>,

    pub status: u8,
    pub property_count: u16,

    #[br(count = property_count)]
    pub properties: Vec<RawProperty>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawScriptList {
    pub version: u16,
    pub object_format: u16,
    pub script_count: u16,

    #[br(count = script_count)]
    pub scripts: Vec<RawScript>,
}

impl TryFrom<VMAD> for RawScriptList {
    type Error = Error;

    fn try_from(raw: VMAD) -> Result<RawScriptList, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        Ok(RawScriptList::read(&mut cursor)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub kind: PropertyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub name: String,
    pub status: u8,
    pub properties: Vec<Property>,
}

impl TryInto<Script> for RawScript {
    type Error = Error;

    fn try_into(self) -> Result<Script, Self::Error> {
        Ok(Script {
            name: String::from_utf8_lossy(&self.name).into(),
            status: self.status,
            properties: self
                .properties
                .into_iter()
                .map(|p| Property {
                    name: String::from_utf8_lossy(&p.name).into(),
                    kind: p.kind,
                })
                .collect(),
        })
    }
}

impl TryInto<Vec<Script>> for VMAD {
    type Error = Error;

    fn try_into(self) -> Result<Vec<Script>, Self::Error> {
        let mut cursor = Cursor::new(&self.data);
        let scripts = RawScriptList::read(&mut cursor)?;
        scripts.scripts.into_iter().map(TryInto::try_into).collect()
    }
}
