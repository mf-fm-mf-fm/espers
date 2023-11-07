use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    Model, ObjectBounds, ScriptList, CNTO, COCT, COED, DATA, EDID, FULL, MODL, MODS, MODT, OBND,
    QNAM, SNAM, VMAD,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [CONT](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/CONT) record
#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"CONT")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CONT {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

/// Parsed [CONT] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model: Option<Model>,
    pub object_count: Option<u32>,
    pub objects: Vec<(FormID, u32)>,
    pub unknown: Option<(FormID, FormID, f32)>,
    pub flags: DATA,
    pub open_sound: Option<FormID>,
    pub close_sound: Option<FormID>,
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Container ({})", self.edid)
    }
}

impl TryFrom<CONT> for Container {
    type Error = Error;

    fn try_from(raw: CONT) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let scripts = VMAD::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let full_name = match (FULL::read(&mut cursor), raw.localized) {
            (Ok(f), true) => Some(LocalizedString::Localized(f.try_into()?)),
            (Ok(z), false) => Some(LocalizedString::ZString(z.try_into()?)),
            (Err(_), _) => None,
        };
        let model = Model::try_load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;
        let object_count = COCT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut objects = Vec::new();
        let mut unknown = None;
        loop {
            if let Ok(c) = CNTO::read(&mut cursor) {
                objects.push(c.try_into()?);
                continue;
            }
            if let Ok(c) = COED::read(&mut cursor) {
                unknown = Some(c.try_into()?);
                continue;
            }

            break;
        }
        let flags = DATA::read(&mut cursor)?;
        let open_sound = SNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let close_sound = QNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            scripts,
            bounds,
            full_name,
            model,
            object_count,
            objects,
            unknown,
            flags,
            open_sound,
            close_sound,
        })
    }
}
