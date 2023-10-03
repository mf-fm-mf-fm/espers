use crate::common::FormID;
use crate::error::Error;
use crate::records::{tes4::Flags, Header, RawRecord, Record, TES4};
use binrw::{until_eof, BinRead, Endian};
use std::collections::HashMap;
use std::io::{Read, Seek};

type RecordKey = Vec<usize>;

#[derive(Debug)]
pub struct Plugin {
    pub header: Header,
    pub records: Vec<Result<Record, Error>>,
    pub form_ids: HashMap<u32, RecordKey>,
}

fn helper(rec: &Result<Record, Error>, path: Vec<usize>) -> Vec<(u32, RecordKey)> {
    match rec {
        Ok(Record::Group(g)) => g
            .records
            .iter()
            .enumerate()
            .flat_map(|(i, r)| {
                let mut p = path.clone();
                p.push(i);
                helper(r, p)
            })
            .collect(),
        Ok(r) => vec![(r.form_id().unwrap(), path)],
        Err(_) => vec![],
    }
}

impl Plugin {
    pub fn parse<T: Read + Seek>(reader: &mut T) -> Result<Self, Error> {
        let header: Header = TES4::read(reader)?.try_into()?;
        let args = (header.header.flags.contains(Flags::LOCALIZED),);
        let recs: Vec<RawRecord> = until_eof(reader, Endian::Little, args)?;

        let records: Vec<_> = recs.into_iter().map(Record::try_from).collect();
        let form_ids = records
            .iter()
            .enumerate()
            .flat_map(|(i, r)| helper(r, vec![i]))
            .collect();

        Ok(Self {
            header,
            records,
            form_ids,
        })
    }

    pub fn get_record_by_key(&self, key: &RecordKey) -> Option<&Result<Record, Error>> {
        let mut selected: Option<&Result<Record, Error>> = None;
        for i in key {
            selected = match selected {
                Some(Ok(Record::Group(g))) => Some(&g.records[*i]),
                Some(_) => unreachable!("This should not happen!"),
                None => Some(&self.records[*i]),
            }
        }
        selected
    }

    pub fn get_record_by_form_id(&self, fid: &FormID) -> Option<&Result<Record, Error>> {
        self.form_ids
            .get(&fid.0)
            .and_then(|fid| self.get_record_by_key(&fid))
    }
}
