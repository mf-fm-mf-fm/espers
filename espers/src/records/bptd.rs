use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID, LocalizedString};
use crate::error::Error;
use crate::fields::{
    Model, BPND, BPNI, BPNN, BPNT, BPTN, EDID, MODL, MODS, MODT, NAM1, NAM4, NAM5, RAGA,
};
use binrw::{binrw, BinRead};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[br(import(localized: bool))]
#[brw(little, magic = b"BPTD")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BPTD {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,

    #[br(calc(localized))]
    #[bw(ignore)]
    pub localized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPart {
    pub name: LocalizedString,
    pub node_name: String,
    pub node_title: String,
    pub node_info: String,
    pub node_data: [u32; 21],
    pub limb_replacement_model: String,
    pub gore_effects: String,
    pub hashes: Vec<u8>,
    pub ragdoll: Option<FormID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPartData {
    pub header: RecordHeader,
    pub edid: String,
    pub model: Option<Model>,
    pub body_parts: Vec<BodyPart>,
}

impl fmt::Display for BodyPartData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BodyPartData ({})", self.edid)
    }
}

impl TryFrom<BPTD> for BodyPartData {
    type Error = Error;

    fn try_from(raw: BPTD) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let model = Model::try_load::<MODL, MODT, MODS>(&mut cursor, raw.header.internal_version)?;

        let mut body_parts = Vec::new();
        while let Ok(bptn) = BPTN::read(&mut cursor) {
            let name = if raw.localized {
                LocalizedString::Localized(bptn.try_into()?)
            } else {
                LocalizedString::ZString(bptn.try_into()?)
            };
            let node_name = BPNN::read(&mut cursor)?.try_into()?;
            let node_title = BPNT::read(&mut cursor)?.try_into()?;
            let node_info = BPNI::read(&mut cursor)?.try_into()?;
            let node_data = BPND::read(&mut cursor)?.try_into()?;
            let limb_replacement_model = NAM1::read(&mut cursor)?.try_into()?;
            let gore_effects = NAM4::read(&mut cursor)?.try_into()?;
            let hashes = NAM5::read(&mut cursor)?.try_into()?;
            let ragdoll = RAGA::read(&mut cursor)
                .ok()
                .map(TryInto::try_into)
                .transpose()?;

            body_parts.push(BodyPart {
                name,
                node_name,
                node_title,
                node_info,
                node_data,
                limb_replacement_model,
                gore_effects,
                hashes,
                ragdoll,
            })
        }

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            model,
            body_parts,
        })
    }
}
