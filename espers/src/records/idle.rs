use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{EffectCondition, ANAM, DATA, DNAM, EDID, ENAM};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// [IDLE](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format/IDLE) record
#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"IDLE")]
pub struct IDLE {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct IdleDataFlags: u8 {
        const IDLE_PARENT = 0x01;
        const IDLE_SEQUENCE = 0x02;
        const NO_ATTACKING = 0x04;
        const IDLE_BLOCKING = 0x08;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdleData {
    pub min_looping_seconds: u8,
    pub max_looping_seconds: u8,
    pub flags: IdleDataFlags,
    pub animation_group_section: u8,
    pub replay_delay: u16,
}

impl TryFrom<DATA> for IdleData {
    type Error = Error;

    fn try_from(raw: DATA) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read_le(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

/// Parsed [IDLE] record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdleAnimation {
    pub header: RecordHeader,
    pub edid: Option<String>,
    pub conditions: Vec<EffectCondition>,
    pub havok_file: Option<String>,
    pub animation_event: Option<String>,
    pub animations: (FormID, FormID),
    pub data: IdleData,
}

impl fmt::Display for IdleAnimation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IdleAnimation ({})", self.edid.as_deref().unwrap_or("~"))
    }
}

impl TryFrom<IDLE> for IdleAnimation {
    type Error = Error;

    fn try_from(raw: IDLE) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let conditions = EffectCondition::load_multiple(&mut cursor)?;
        let havok_file = DNAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let animation_event = ENAM::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let animations = ANAM::read(&mut cursor)?.try_into()?;
        let data = DATA::read(&mut cursor)?.try_into()?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            conditions,
            havok_file,
            animation_event,
            animations,
            data,
        })
    }
}
