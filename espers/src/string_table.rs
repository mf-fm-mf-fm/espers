use crate::error::Error;
use binrw::{binrw, BinRead};
use encoding_rs::WINDOWS_1252;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::PathBuf;
use std::str::from_utf8;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little)]
pub struct DirectoryEntry {
    pub string_id: u32,
    pub offset: u32,
}

#[binrw]
#[br(import(length_prefixed: bool))]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawStringTable {
    #[br(calc(length_prefixed))]
    #[bw(ignore)]
    pub length_prefixed: bool,

    pub num_entries: u32,
    pub string_data_size: u32,

    #[br(count = num_entries)]
    pub entries: Vec<DirectoryEntry>,

    #[br(count = string_data_size)]
    pub data: Vec<u8>,
}

impl RawStringTable {
    pub fn parse<T: Read + Seek>(reader: &mut T, length_prefixed: bool) -> Result<Self, Error> {
        Ok(Self::read_args(reader, (length_prefixed,))?)
    }
}

fn get_str(raw: &RawStringTable, offset: usize, length_prefixed: bool) -> Result<String, Error> {
    let bytes = if length_prefixed {
        let mut cursor = Cursor::new(&raw.data[offset..offset + 4]);
        let length = u32::read_le(&mut cursor)? as usize;

        &raw.data[offset + 4..offset + 4 + length]
    } else {
        let end = raw.data[offset..]
            .iter()
            .position(|&c| c == b'\0')
            .ok_or(Error::StringEOF)?;

        &raw.data[offset..offset + end]
    };

    match from_utf8(bytes) {
        Ok(s) => Ok(s.into()),
        Err(_) => {
            let (s, _, _) = WINDOWS_1252.decode(bytes);
            Ok(s.into())
        }
    }
}

pub struct StringTable {
    strings: HashMap<u32, String>,
}

impl StringTable {
    pub fn from_plugin_path(path: &str, language: &str) -> Result<Self, Error> {
        let mut strings = HashMap::new();
        let path = PathBuf::from(path);
        let plugin_name = path.file_stem().unwrap().to_string_lossy();
        let dir = path.parent().unwrap().join("Strings");

        for suffix in ["STRINGS", "DLSTRINGS", "ILSTRINGS"] {
            let y = dir.join(format!("{}_{}.{}", plugin_name, language, suffix));

            let length_prefixed = suffix == "DLSTRINGS" || suffix == "ILSTRINGS";
            if let Ok(mut f) = File::open(y) {
                if let Ok(rst) = RawStringTable::parse(&mut f, length_prefixed) {
                    for entry in &rst.entries {
                        strings.insert(
                            entry.string_id,
                            get_str(&rst, entry.offset as usize, length_prefixed)?,
                        );
                    }
                }
            }
        }

        Ok(Self { strings })
    }

    pub fn get_string(&self, id: &u32) -> Option<&String> {
        self.strings.get(id)
    }
}
