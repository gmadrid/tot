use super::Result;
use crate::kv::from_str;
use crate::{Record, Records};
use std::io::{BufRead, BufReader, Read};

pub fn read_records_from_chunks(
    read: impl Read,
    input_separator: &str,
    trim: bool,
) -> Result<Records> {
    let lines = BufReader::new(read).lines().flatten();

    let chunks = crate::chunker::chunks_at_blanks(lines);
    let mut records: Vec<Record> = Default::default();
    for chunk in chunks {
        let record = record_from_chunk(chunk, input_separator, trim);
        records.push(record);
    }
    Ok(records)
}

fn record_from_chunk<I>(chunk: I, input_separator: &str, trim: bool) -> Record
where
    I: Iterator<Item = String>,
{
    let mut record = Record::default();
    for line in chunk {
        let kv = from_str(&line, input_separator);
        let value = if trim { kv.value().trim() } else { kv.value() };
        record.insert(kv.key().to_string(), value.to_string());
    }
    record
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_create() {
    //     Tot::default();
    // }
    //
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
