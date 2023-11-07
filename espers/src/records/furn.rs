use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    DestructionData, Model, ObjectBounds, ScriptList, EDID, ENAM, FNAM, FNMK, FNPR, FULL, KNAM,
    KSIZ, KWDA, MNAM, MODL, MODS, MODT, NAM0, OBND, PNAM, VMAD, WBDT, XMRK,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [FURN](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/FURN) record
#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"FURN")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FURN {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchData {
    pub kind: u8,
    pub skill: u8,
}

impl TryFrom<WBDT> for WorkbenchData {
    type Error = Error;

    fn try_from(raw: WBDT) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub index: u32,
    pub flags: Option<(u16, u16)>,
    pub keyword: Option<FormID>,
}

impl Marker {
    pub fn load(cursor: &mut Cursor<&Vec<u8>>) -> Result<Self, Error> {
        let index = ENAM::read(cursor)?.try_into()?;
        let flags = NAM0::read(cursor).ok().map(TryInto::try_into).transpose()?;
        let keyword = FNMK::read(cursor).ok().map(TryInto::try_into).transpose()?;

        Ok(Self {
            index,
            flags,
            keyword,
        })
    }

    pub fn load_multiple(cursor: &mut Cursor<&Vec<u8>>) -> Result<Vec<Self>, Error> {
        let mut markers = Vec::new();
        while let Ok(m) = Marker::load(cursor) {
            markers.push(m);
        }
        Ok(markers)
    }
}

/// Parsed [FURN] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Furniture {
    pub header: RecordHeader,
    pub edid: String,
    pub scripts: Option<ScriptList>,
    pub bounds: ObjectBounds,
    pub full_name: Option<LocalizedString>,
    pub model: Option<Model>,
    pub destruction_data: Option<DestructionData>,
    pub keywords: Vec<FormID>,
    pub color: u32,
    pub flags: u16,
    pub interaction_keyword: Option<FormID>,
    pub marker_flags: u32,
    pub workbench_data: WorkbenchData,
    pub markers: Vec<Marker>,
    pub marker_flags_2: Vec<(u16, u16)>,
    pub marker_model: Option<String>,
}

impl fmt::Display for Furniture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Furniture ({})", self.edid)
    }
}

impl TryFrom<FURN> for Furniture {
    type Error = Error;

    fn try_from(raw: FURN) -> Result<Self, Self::Error> {
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
        let destruction_data = DestructionData::load(&mut cursor)?;
        let _: Option<u32> = KSIZ::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let mut keywords = Vec::new();
        while let Ok(kwda) = KWDA::read(&mut cursor) {
            let items: Vec<_> = kwda.try_into()?;
            keywords.extend(items);
        }
        let color = PNAM::read(&mut cursor)?.try_into()?;
        let flags = FNAM::read(&mut cursor)?.try_into()?;
        let interaction_keyword = KNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let marker_flags = MNAM::read(&mut cursor)?.try_into()?;
        let workbench_data = WBDT::read(&mut cursor)?.try_into()?;
        let markers = Marker::load_multiple(&mut cursor)?;
        let mut marker_flags_2 = Vec::new();
        while let Ok(mf) = FNPR::read(&mut cursor) {
            marker_flags_2.push(mf.try_into()?)
        }
        let marker_model = XMRK::read(&mut cursor)
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
            destruction_data,
            keywords,
            color,
            flags,
            interaction_keyword,
            marker_flags,
            workbench_data,
            markers,
            marker_flags_2,
            marker_model,
        })
    }
}
