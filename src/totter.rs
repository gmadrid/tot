use super::Result;
use crate::kv::from_str;
use crate::{Record, Records};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Default)]
pub struct Tot {
    records: Vec<HashMap<String, String>>,
}

impl Tot {
    pub fn read_from(read: impl Read) -> Result<Tot> {
        let lines = BufReader::new(read).lines().flatten(); //.map(|s| s.as_str());
                                                            //let foo: u8 = lines.next().unwrap();
        let chunks = crate::chunker::chunks_at_blanks(lines);
        let mut records: Vec<Record> = Default::default();
        for chunk in chunks {
            let record = Tot::record_from_chunk(chunk);
            records.push(record);
        }
        Ok(Tot { records })
    }

    fn record_from_chunk<I>(chunk: I) -> Record
    where
        I: Iterator<Item = String>,
    {
        let mut record = Record::default();
        for line in chunk {
            let kv = from_str(&line);
            record.insert(kv.key().to_string(), kv.value().to_string());
        }
        record
    }

    pub fn take_records(self) -> Result<Records> {
        Ok(self.records)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create() {
        Tot::default();
    }

    //     #[test]
    //     fn test_simple_chunks() {
    //         let input = r#"one:1
    // two:2
    // three:3
    //
    // one:o
    // two:t
    // three:t"#;
    //
    //         let tot = Tot::read_from(input.as_bytes()).unwrap();
    //
    //         let records = tot.take_records().unwrap();
    //
    //         assert_eq!(2, records.len());
    // }

    // test blanks with white space in them.
    // test lines with no separator
}
