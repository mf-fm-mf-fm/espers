use crate::common::check_done_reading;
use crate::error::Error;
use crate::records::{RawRecord, Record};
use binrw::{binrw, io::Cursor, until_eof, Endian};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

bitflags! {
    #[binrw]
    #[derive(Deserialize, Serialize)]
    struct Flags: u32 {
        const MASTER = 0x00000001;
        const LOCALIZED = 0x00000080;
        const LIGHT_MASTER = 0x00000200;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupHeader {
    pub size: u32,
    pub label: u32,
    pub kind: i32,
    pub timestamp: u16,
    pub version_control_info: u16,
    pub unknown: u32,
}

/// [GRUP](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/GRUP) record
#[binrw]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[brw(little, magic = b"GRUP")]
#[br(import(localized: bool))]
pub struct GRUP {
    pub header: GroupHeader,

    #[br(count = header.size - 24)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

/// Parsed [GRUP] record
#[derive(Debug)]
pub struct Group {
    pub header: GroupHeader,
    pub records: Vec<Result<Record, Error>>,
}

impl Group {
    pub fn magics(&self) -> Vec<String> {
        let mut magics = Vec::new();

        for rec in &self.records {
            match rec {
                Ok(rec) => {
                    let magic = String::from_utf8_lossy(&rec.magic()).to_string();
                    if !magics.contains(&magic) {
                        magics.push(magic);
                    }
                }
                Err(_) => continue,
            }
        }

        magics
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group ({} items)", self.records.len())
    }
}

impl TryFrom<GRUP> for Group {
    type Error = Error;

    fn try_from(raw: GRUP) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let args = (raw.localized,);
        let recs: Vec<RawRecord> = until_eof(&mut cursor, Endian::Little, args)?;
        let records: Vec<_> = recs
            .into_iter()
            .map(|r| {
                r.clone().try_into().map_err(|err| match err {
                    Error::BinaryParseError(e) => Error::BinaryParseErrorExtra(e, r),
                    Error::ExtraBytes(e) => Error::ExtraBytesRaw(e, r),
                    other => other,
                })
            })
            .collect();

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            records,
        })
    }
}

impl TryFrom<Group> for GRUP {
    type Error = Error;

    fn try_from(obj: Group) -> Result<Self, Self::Error> {
        let data = Cursor::new(Vec::new());

        Ok(Self {
            header: obj.header,
            data: data.into_inner(),
            localized: false,
        })
    }
}
