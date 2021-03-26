use std::io::{BufRead, BufReader, Read};

mod chunker;
mod kv;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IoError: {0:?}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub type Record = std::collections::HashMap<String, String>;
pub type Records = Vec<Record>;

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
        let kv = kv::from_str(&line, input_separator);
        let value = if trim { kv.value().trim() } else { kv.value() };
        record.insert(kv.key().to_string(), value.to_string());
    }
    record
}
