use binrw::BinWrite;
use binrw::__private::Required;
use glob::glob;
use std::io::Cursor;

fn serialize<T: BinWrite>(rec: &T) -> Vec<u8>
where
    for<'a> <T as BinWrite>::Args<'a>: Required,
{
    let mut buf = Cursor::new(Vec::new());
    rec.write_le(&mut buf).unwrap();
    buf.into_inner()
}

fn list_dir() -> Vec<String> {
    glob("assets/skyrim/*.es[mp]")
        .unwrap()
        .into_iter()
        .map(|f| format!("{}", f.unwrap().display()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{list_dir, serialize};
    use binrw::BinRead;
    use espers::records::{Header, TES4};
    use std::fs::File;
    use std::io::{Cursor, Read};

    #[test]
    /// Tests that reading and writing end up matching per-bit
    ///
    /// Read the raw bytes for a given record, parse it into a RawRecord, then
    /// parse that into a Record. Next, serialize the RawRecord back to bytes,
    /// and the record back to a RawRecord, then back to bytes. Assert that this
    /// entire process resulted in identical bits.
    pub fn test_read_write_match() {
        for file in list_dir() {
            let mut reader = File::open(file).unwrap();
            let mut bytes = vec![0; 24];
            reader.read_exact(&mut bytes).unwrap();
            let size = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
            let mut data = vec![0; size as usize];
            reader.read_exact(&mut data).unwrap();
            bytes.extend(data);

            let raw = TES4::read(&mut Cursor::new(&bytes)).unwrap();
            let raw_record = serialize(&raw);

            let header: Header = raw.try_into().unwrap();
            let reraw: TES4 = header.try_into().unwrap();

            let record = serialize(&reraw);

            assert_eq!(bytes, raw_record);
            assert_eq!(raw_record, record);
        }
    }
}
