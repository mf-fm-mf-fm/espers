use super::{get_cursor, Flags};
use crate::common::{FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    ModelTextures, ObjectBounds, Script, CNAM, DATA, DESC, EDID, FULL, ICON, INAM, KSIZ, KWDA,
    MICO, MODL, MODT, OBND, VMAD, YNAM, ZNAM,
};
use crate::string_table::StringTables;
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct BookFlags: u8 {
        const TEACHES_SKILL = 0x01;
        const CANNOT_BE_TAKEN = 0x02;
        const TEACHES_SPELL = 0x04;
        const READ = 0x08;
    }
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little)]
pub struct BookData {
    pub flags: BookFlags,
    pub kind: u8,
    pub unknown: [u8; 2],
    pub teaches: FormID,
    pub value: u32,
    pub weight: f32,
}

impl TryFrom<DATA> for BookData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        Ok(Self::read(&mut Cursor::new(&raw.data))?)
    }
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"BOOK")]
#[br(import(localized: bool))]
pub struct BOOK {
    pub size: u32,
    pub flags: Flags,
    pub form_id: u32,
    pub timestamp: u16,
    pub version_control: u16,
    pub internal_version: u16,
    pub unknown: u16,
    #[br(count = size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub edid: String,
    pub scripts: Vec<Script>,
    pub obnd: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model_filename: Option<String>,
    pub model_textures: Option<ModelTextures>,
    pub text: String,
    pub icon: Option<String>,
    pub message_icon: Option<String>,
    pub pickup_sound: Option<FormID>,
    pub drop_sound: Option<FormID>,
    pub keywords: Vec<FormID>,
    pub data: BookData,
    pub inventory_art: Option<FormID>,
    pub description: String,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Book ({})", self.edid)
    }
}

impl TryFrom<BOOK> for Book {
    type Error = Error;

    fn try_from(raw: BOOK) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?
            .unwrap_or_default();
        let obnd = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        // let full_name = FULL::read(&mut cursor)
        //     .ok()
        //     .map(TryInto::try_into)
        //     .transpose()?;
        let model_filename = MODL::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let model_textures = MODT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let text = DESC::read(&mut cursor)?.try_into()?;
        let icon = ICON::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let message_icon = MICO::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let pickup_sound = YNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let drop_sound = ZNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        let keyword_count: Option<u32> = KSIZ::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut keywords = Vec::new();

        if let Some(kc) = keyword_count {
            for _ in 0..kc {
                // It's actually only up to keyword count
                if let Ok(kwda) = KWDA::read(&mut cursor) {
                    keywords.push(FormID::read_le(&mut Cursor::new(kwda.data)).unwrap());
                }
            }
        }

        let data = DATA::read(&mut cursor)?.try_into()?;
        let inventory_art = INAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let description = CNAM::read(&mut cursor)?.try_into()?;

        Ok(Self {
            edid,
            scripts,
            obnd,
            full_name,
            model_filename,
            model_textures,
            text,
            icon,
            message_icon,
            pickup_sound,
            drop_sound,
            keywords,
            data,
            inventory_art,
            description,
        })
    }
}

impl Book {
    pub fn localize(&mut self, string_table: &StringTables) {
        if let Some(LocalizedString::Localized(l)) = self.full_name {
            if let Ok(Some(s)) = string_table.get_string(&l) {
                self.full_name = Some(LocalizedString::ZString(s.to_string()));
            }
        }
    }
}
