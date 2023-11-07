use super::{get_cursor, Flags, RecordHeader};
use crate::common::check_done_reading;
use crate::error::Error;
use crate::fields::{
    BNAM, DNAM, EDID, NAM1, NAM2, NAM3, NAM4, RNAM, SNAM, TNAM, UNAM, VNAM, WNAM, XNAM, YNAM,
};
use binrw::{binrw, io::Cursor, until_eof, BinRead, Endian};
use bitflags::bitflags;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::Read;

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[brw(little, magic = b"IMAD")]
pub struct IMAD {
    pub header: RecordHeader,

    #[br(count = header.size)]
    pub data: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    pub timestamp: f32,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamps(Vec<Timestamp>);

impl TryFrom<Vec<u8>> for Timestamps {
    type Error = Error;

    fn try_from(raw: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw);
        Ok(Self(until_eof(&mut cursor, Endian::Little, ())?))
    }
}

fn read_attr(cursor: &mut Cursor<&Vec<u8>>) -> Result<Timestamps, Error> {
    <[u8; 4]>::read_le(cursor)
        .map_err(Error::BinaryParseError)
        .and_then(|_| {
            let len = u16::read_le(cursor)?;
            let mut buf = vec![0u8; len as usize];
            cursor.read_exact(&mut buf)?;
            buf.try_into()
        })
}

bitflags! {
    #[binrw]
    #[brw(little)]
    #[derive(Deserialize, Serialize)]
    pub struct ImageSpaceAdapterFlags: u32 {
        const ANIMATABLE = 0x01;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSpaceAdapterData {
    pub flags: ImageSpaceAdapterFlags,
    pub duration: f32,
    pub entry_counts: Vec<u32>,
}

impl TryFrom<DNAM> for ImageSpaceAdapterData {
    type Error = Error;

    fn try_from(raw: DNAM) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&raw.data);
        let flags = BinRead::read(&mut cursor)?;
        let duration = f32::read_le(&mut cursor)?;
        let entry_counts = until_eof(&mut cursor, Endian::Little, ())?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            flags,
            duration,
            entry_counts,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSpaceAdapter {
    pub header: RecordHeader,
    pub edid: String,
    pub data: ImageSpaceAdapterData,
    pub blur_radius: Timestamps,
    pub double_vision_strength: Timestamps,
    pub radial_blur_strength: Timestamps,
    pub radial_blur_rampup: Timestamps,
    pub radial_blur_start: Timestamps,
    pub radial_blur_rampdown: Timestamps,
    pub radial_blur_downstart: Timestamps,
    pub depth_of_field_strength: Timestamps,
    pub depth_of_field_distance: Timestamps,
    pub depth_of_field_range: Timestamps,
    pub fullscreen_motion_blur: Timestamps,
    pub eye_adapt_speed_multiply: Timestamps,
    pub eye_adapt_speed_add: Timestamps,
    pub bloom_blur_radius_multiply: Timestamps,
    pub bloom_blur_radius_add: Timestamps,
    pub bloom_threshold_multiply: Timestamps,
    pub bloom_threshold_add: Timestamps,
    pub bloom_scale_multiply: Timestamps,
    pub bloom_scale_add: Timestamps,
    pub target_lum_min_multiply: Timestamps,
    pub target_lum_min_add: Timestamps,
    pub target_lum_max_multiply: Timestamps,
    pub target_lum_max_add: Timestamps,
    pub sunlight_scale_multiply: Timestamps,
    pub sunlight_scale_add: Timestamps,
    pub sky_scale_multiply: Timestamps,
    pub sky_scale_add: Timestamps,
    pub unknown_1: Timestamps,
    pub unknown_2: Timestamps,
    pub unknown_3: Timestamps,
    pub unknown_4: Timestamps,
    pub unknown_5: Timestamps,
    pub unknown_6: Timestamps,
    pub unknown_7: Timestamps,
    pub unknown_8: Timestamps,
    pub unknown_9: Timestamps,
    pub unknown_10: Timestamps,
    pub unknown_11: Timestamps,
    pub unknown_12: Timestamps,
    pub unknown_13: Timestamps,
    pub unknown_14: Timestamps,
    pub unknown_15: Timestamps,
    pub unknown_16: Timestamps,
    pub unknown_17: Timestamps,
    pub unknown_18: Timestamps,
    pub saturation_multiply: Timestamps,
    pub saturation_add: Timestamps,
    pub brightness_multiply: Timestamps,
    pub brightness_add: Timestamps,
    pub contrast_multiply: Timestamps,
    pub contrast_add: Timestamps,
    pub unknown_19: Timestamps,
    pub unknown_20: Timestamps,
}

impl fmt::Display for ImageSpaceAdapter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImageSpaceModifier ({})", self.edid)
    }
}

impl TryFrom<IMAD> for ImageSpaceAdapter {
    type Error = Error;

    fn try_from(raw: IMAD) -> Result<Self, Self::Error> {
        let data = get_cursor(&raw.data, raw.header.flags.contains(Flags::COMPRESSED));
        let mut cursor = Cursor::new(&data);

        let edid = EDID::read(&mut cursor)?.try_into()?;
        let data = DNAM::read(&mut cursor)?.try_into()?;
        let blur_radius = BNAM::read(&mut cursor)?.data.try_into()?;
        let double_vision_strength = VNAM::read(&mut cursor)?.data.try_into()?;
        let _ = TNAM::read(&mut cursor)?;
        let _ = NAM3::read(&mut cursor)?;
        let radial_blur_strength = RNAM::read(&mut cursor)?.data.try_into()?;
        let radial_blur_rampup = SNAM::read(&mut cursor)?.data.try_into()?;
        let radial_blur_start = UNAM::read(&mut cursor)?.data.try_into()?;
        let radial_blur_rampdown = NAM1::read(&mut cursor)?.data.try_into()?;
        let radial_blur_downstart = NAM2::read(&mut cursor)?.data.try_into()?;
        let depth_of_field_strength = WNAM::read(&mut cursor)?.data.try_into()?;
        let depth_of_field_distance = XNAM::read(&mut cursor)?.data.try_into()?;
        let depth_of_field_range = YNAM::read(&mut cursor)?.data.try_into()?;
        let fullscreen_motion_blur = NAM4::read(&mut cursor)?.data.try_into()?;

        let eye_adapt_speed_multiply = read_attr(&mut cursor)?;
        let eye_adapt_speed_add = read_attr(&mut cursor)?;
        let bloom_blur_radius_multiply = read_attr(&mut cursor)?;
        let bloom_blur_radius_add = read_attr(&mut cursor)?;
        let bloom_threshold_multiply = read_attr(&mut cursor)?;
        let bloom_threshold_add = read_attr(&mut cursor)?;
        let bloom_scale_multiply = read_attr(&mut cursor)?;
        let bloom_scale_add = read_attr(&mut cursor)?;
        let target_lum_min_multiply = read_attr(&mut cursor)?;
        let target_lum_min_add = read_attr(&mut cursor)?;
        let target_lum_max_multiply = read_attr(&mut cursor)?;
        let target_lum_max_add = read_attr(&mut cursor)?;
        let sunlight_scale_multiply = read_attr(&mut cursor)?;
        let sunlight_scale_add = read_attr(&mut cursor)?;
        let sky_scale_multiply = read_attr(&mut cursor)?;
        let sky_scale_add = read_attr(&mut cursor)?;
        let unknown_1 = read_attr(&mut cursor)?;
        let unknown_2 = read_attr(&mut cursor)?;
        let unknown_3 = read_attr(&mut cursor)?;
        let unknown_4 = read_attr(&mut cursor)?;
        let unknown_5 = read_attr(&mut cursor)?;
        let unknown_6 = read_attr(&mut cursor)?;
        let unknown_7 = read_attr(&mut cursor)?;
        let unknown_8 = read_attr(&mut cursor)?;
        let unknown_9 = read_attr(&mut cursor)?;
        let unknown_10 = read_attr(&mut cursor)?;
        let unknown_11 = read_attr(&mut cursor)?;
        let unknown_12 = read_attr(&mut cursor)?;
        let unknown_13 = read_attr(&mut cursor)?;
        let unknown_14 = read_attr(&mut cursor)?;
        let unknown_15 = read_attr(&mut cursor)?;
        let unknown_16 = read_attr(&mut cursor)?;
        let unknown_17 = read_attr(&mut cursor)?;
        let unknown_18 = read_attr(&mut cursor)?;
        let saturation_multiply = read_attr(&mut cursor)?;
        let saturation_add = read_attr(&mut cursor)?;
        let brightness_multiply = read_attr(&mut cursor)?;
        let brightness_add = read_attr(&mut cursor)?;
        let contrast_multiply = read_attr(&mut cursor)?;
        let contrast_add = read_attr(&mut cursor)?;
        let unknown_19 = read_attr(&mut cursor)?;
        let unknown_20 = read_attr(&mut cursor)?;

        check_done_reading(&mut cursor)?;

        Ok(Self {
            header: raw.header,
            edid,
            data,
            blur_radius,
            double_vision_strength,
            radial_blur_strength,
            radial_blur_rampup,
            radial_blur_start,
            radial_blur_rampdown,
            radial_blur_downstart,
            depth_of_field_strength,
            depth_of_field_distance,
            depth_of_field_range,
            fullscreen_motion_blur,
            eye_adapt_speed_multiply,
            eye_adapt_speed_add,
            bloom_blur_radius_multiply,
            bloom_blur_radius_add,
            bloom_threshold_multiply,
            bloom_threshold_add,
            bloom_scale_multiply,
            bloom_scale_add,
            target_lum_min_multiply,
            target_lum_min_add,
            target_lum_max_multiply,
            target_lum_max_add,
            sunlight_scale_multiply,
            sunlight_scale_add,
            sky_scale_multiply,
            sky_scale_add,
            unknown_1,
            unknown_2,
            unknown_3,
            unknown_4,
            unknown_5,
            unknown_6,
            unknown_7,
            unknown_8,
            unknown_9,
            unknown_10,
            unknown_11,
            unknown_12,
            unknown_13,
            unknown_14,
            unknown_15,
            unknown_16,
            unknown_17,
            unknown_18,
            saturation_multiply,
            saturation_add,
            brightness_multiply,
            brightness_add,
            contrast_multiply,
            contrast_add,
            unknown_19,
            unknown_20,
        })
    }
}
