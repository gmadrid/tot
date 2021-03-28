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
    let parser = kv::KvParser::with_separator(input_separator);

    let lines = BufReader::new(read).lines().flatten();

    Ok(crate::chunker::chunks_at_blanks(lines)
        .map(|chunk| record_from_chunk(chunk, &parser, trim))
        .collect())
}

fn record_from_chunk<I>(chunk: I, parser: &kv::KvParser, trim: bool) -> Record
where
    I: Iterator<Item = String>,
{
    chunk.fold(Record::default(), |mut record, line| {
        let kv = parser.parse(&line);
        let value = if trim { kv.value().trim() } else { kv.value() };
        record.insert(kv.key().to_string(), value.to_string());
        record
    })
}
