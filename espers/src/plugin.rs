use crate::error::Error;
use crate::records::{tes4::Flags, Header, RawRecord, Record, TES4};
use crate::string_table::StringTables;
use binrw::{until_eof, BinRead, Endian};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct Plugin {
    pub header: Header,
    pub records: Vec<Record>,
}

impl Plugin {
    pub fn parse<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        let header: Header = TES4::read(reader)?.try_into()?;
        let args = (header.header.flags.contains(Flags::LOCALIZED),);
        let recs: Vec<RawRecord> = until_eof(reader, Endian::Little, args)?;

        let records: Result<Vec<Record>, _> = recs.into_iter().map(Record::try_from).collect();

        Ok(Self {
            header,
            records: records?,
        })
    }

    pub fn localize(&mut self, string_table: &StringTables) {
        for record in &mut self.records {
            record.localize(string_table);
        }
    }
}
