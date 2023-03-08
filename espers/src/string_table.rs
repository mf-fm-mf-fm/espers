use crate::error::Error;
use binrw::{binrw, BinRead};
use encoding_rs::WINDOWS_1252;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little)]
pub struct RawStringTable {
    pub num_entries: u32,
    pub string_data_size: u32,

    #[br(count = num_entries)]
    pub entries: Vec<DirectoryEntry>,

    #[br(count = string_data_size)]
    pub data: Vec<u8>,
}

impl RawStringTable {
    pub fn parse<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        Ok(Self::read(reader)?)
    }
}

pub struct StringTable {
    pub offsets: HashMap<u32, u32>,
    pub data: Vec<u8>,
}

impl From<RawStringTable> for StringTable {
    fn from(raw: RawStringTable) -> Self {
        Self {
            offsets: raw
                .entries
                .into_iter()
                .map(|de| (de.string_id, de.offset))
                .collect(),
            data: raw.data,
        }
    }
}

impl StringTable {
    pub fn get_str(&self, id: &u32) -> Result<Option<String>, Error> {
        let start = match self.offsets.get(id) {
            Some(&v) => v as usize,
            None => return Ok(None),
        };
        let end = self.data[start..]
            .iter()
            .position(|&c| c == b'\0')
            .ok_or(Error::StringEOF)?;

        let bytes = &self.data[start..start + end];

        let result: Result<String, Error> = match from_utf8(bytes) {
            Ok(s) => Ok(s.into()),
            Err(_) => {
                let (s, _, _) = WINDOWS_1252.decode(bytes);
                Ok(s.into())
            }
        };
        Ok(Some(result?))
    }

    pub fn get_slice(&self, id: &u32) -> Result<Option<&[u8]>, Error> {
        let start = match self.offsets.get(id) {
            Some(&v) => v as usize,
            None => return Ok(None),
        };
        let end = self.data[start..]
            .iter()
            .position(|&c| c == b'\0')
            .ok_or(Error::StringEOF)?;

        Ok(Some(&self.data[start..start + end]))
    }
}

pub struct StringTables {
    string_tables: Vec<StringTable>,
}

impl StringTables {
    pub fn load(path: &str, language: &str) -> Result<Self, Error> {
        let path = PathBuf::from(path);
        let plugin_name = path.file_stem().unwrap().to_string_lossy();
        let dir = path.parent().unwrap().join("Strings");

        let mut string_tables = Vec::new();

        for suffix in ["STRINGS", "DLSTRINGS", "ILSTRINGS"] {
            let y = dir.join(format!("{}_{}.{}", plugin_name, language, suffix));

            if let Ok(mut f) = File::open(y) {
                if let Ok(rst) = RawStringTable::parse(&mut f) {
                    string_tables.push(rst.into())
                }
            }
        }

        Ok(Self { string_tables })
    }

    pub fn get_string(&self, id: &u32) -> Result<Option<String>, Error> {
        for st in &self.string_tables {
            if let Some(s) = st.get_str(id)? {
                return Ok(Some(s));
            }
        }

        Ok(None)
    }

    pub fn get_slice(&self, id: &u32) -> Result<Option<&[u8]>, Error> {
        for st in &self.string_tables {
            if let Some(s) = st.get_slice(id)? {
                return Ok(Some(s));
            }
        }

        Ok(None)
    }
}
