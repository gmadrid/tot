use argh::FromArgs;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::Extend;
use std::path::{Path, PathBuf};
use tot::{Record, Records, Result};

#[derive(FromArgs)]
/// Convert tagged records to TAB-delimited.
struct Args {
    #[argh(option, default = "Default::default()")]
    /// columns to be listed first in this order. "first,second,third"
    order: String,

    #[argh(option, default = "\":\".to_string()", short = 's')]
    /// separator between the field name and the value in the input
    input_separator: String,

    #[argh(option, default = "\"\\t\".to_string()", short = 'S')]
    /// field separator used in the output
    output_separator: String,

    #[argh(switch)]
    /// trim whitespace from beginning/end of values
    trim: bool,

    #[argh(positional)]
    /// the file to process. (Currently only a single file is allowed.)
    filename: PathBuf,
}

type KeySet<'a> = HashSet<&'a str>;

fn get_input_records(path: &Path, input_separator: &str, trim: bool) -> Result<Records> {
    let f = std::fs::File::open(path)?;

    let tot = tot::Tot::read_from(f, input_separator, trim)?;
    tot.take_records()
}

fn get_all_key_names(recs: &[Record]) -> KeySet {
    recs.iter().fold(KeySet::default(), |mut set, rec| {
        set.extend(rec.keys().map(|s| s.as_str()));
        set
    })
}

fn get_first_keys(list: &str) -> Vec<&str> {
    if list.is_empty() {
        Default::default()
    } else {
        list.split(',').collect()
    }
}

fn get_ordered_keys<'a>(first: &[&'a str], mut unordered: KeySet<'a>) -> Vec<&'a str> {
    // Remove the first keys from the unordered list.
    first.iter().for_each(|k| {
        unordered.remove(k);
    });

    first.iter().chain(unordered.iter()).copied().collect()
}

fn spew_headers(keys: &[&str], separator: &str) {
    println!("{}", keys.iter().join(separator));
}

fn spew_records(keys: &[&str], records: &[Record], separator: &str) {
    for rec in records {
        println!(
            "{}",
            keys.iter()
                .map(|k| { rec.get(*k).map(|v| v.as_str()).unwrap_or("") })
                .join(separator)
        );
    }
}

fn process() -> Result<()> {
    let args = argh::from_env::<Args>();

    let recs = get_input_records(&args.filename, &args.input_separator, args.trim)?;
    let all_key_set = get_all_key_names(&recs);
    let first_keys = get_first_keys(&args.order);
    let ordered_keys = get_ordered_keys(&first_keys, all_key_set);

    spew_headers(&ordered_keys, &args.output_separator);
    spew_records(&ordered_keys, &recs, &args.output_separator);

    Ok(())
}

fn main() {
    match process() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
