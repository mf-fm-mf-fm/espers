use super::{get_cursor, Flags, RecordHeader};
use crate::common::{check_done_reading, FormID};
use crate::error::Error;
use crate::fields::{ObjectBounds, EDID, IDLA, IDLC, IDLF, IDLT, OBND};
use binrw::{binrw, BinRead};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"IDLM")]
pub struct IDLM {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct IdleMarkerFlags: u8 {
        const ORDERING = 0x01;
        const DO_ONCE = 0x04;
        const UNKNOWN = 0x08;
        const IGNORED_BY_SANDBOX = 0x10;
    }
}

impl TryFrom<IDLF> for IdleMarkerFlags {
    type Error = Error;

    fn try_from(raw: IDLF) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let result = Self::read(&mut cursor)?;
        check_done_reading(&mut cursor)?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdleMarker {
    pub header: RecordHeader,
    pub edid: String,
    pub bounds: ObjectBounds,
    pub flags: Option<IdleMarkerFlags>,
    pub count: Option<u8>,
    pub timer: Option<f32>,
    pub animations: Vec<FormID>,
}

impl fmt::Display for IdleMarker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IdleMarker ({})", self.edid)
    }
}

impl TryFrom<IDLM> for IdleMarker {
    type Error = Error;

    fn try_from(raw: IDLM) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let bounds = OBND::read(&mut cursor)?.try_into()?;
        let flags = IDLF::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let count = IDLC::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let timer = IDLT::read(&mut cursor)
            .ok()
            .map(TryInto::try_into)
            .transpose()?;
        let animations = match IDLA::read(&mut cursor) {
            Ok(i) => i.try_into()?,
            Err(_) => Vec::new(),
        };

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            bounds,
            flags,
            count,
            timer,
            animations,
        })
    }
}
