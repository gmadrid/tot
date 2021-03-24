use argh::FromArgs;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::Extend;
use std::path::{Path, PathBuf};
use tot::{Record, Result};

#[derive(FromArgs)]
/// Convert tagged records to TAB-delimited.
struct Args {
    #[argh(option, default = "Default::default()")]
    /// columns to be listed first in this order. "first,second,third"
    order: String,

    #[argh(positional)]
    /// the file to process. (Currently only a single file is allowed.)
    filename: PathBuf,
}

type KeySet<'a> = HashSet<&'a str>;

fn get_input_records(path: &Path) -> Result<tot::Records> {
    let f = std::fs::File::open(path)?;

    let tot = tot::Tot::read_from(f)?;
    tot.take_records()
}

fn get_all_key_names(recs: &[Record]) -> KeySet {
    recs.iter().fold(KeySet::default(), |mut set, rec| {
        set.extend(rec.keys().map(|s| s.as_str()));
        set
    })
}

fn get_first_keys(list: &str) -> Vec<&str> {
    list.split(',').collect()
}

fn get_ordered_keys<'a>(first: &[&'a str], mut unordered: KeySet<'a>) -> Vec<&'a str> {
    let mut keys_in_order = Vec::default();

    // Add the keys in the first list.
    keys_in_order.extend(first.iter());

    // Remove those first keys from the unordered list.
    first.iter().for_each(|k| {
        unordered.remove(k);
    });

    // Now add the remaining unordered keys to the list.
    keys_in_order.extend(unordered.iter());

    keys_in_order
}

fn spew_headers(keys: &[&str]) {
    println!("{}", keys.iter().join("\t"));
}

fn spew_records(keys: &[&str], records: &[Record]) {
    for rec in records {
        println!(
            "{}",
            keys.iter()
                .map(|k| { rec.get(*k).map(|v| v.as_str()).unwrap_or("") })
                .join("\t")
        );
    }
}

fn process() -> Result<()> {
    let args = argh::from_env::<Args>();
    let recs = get_input_records(&args.filename)?;
    let all_key_set = get_all_key_names(&recs);
    let first_keys = get_first_keys(&args.order);
    let ordered_keys = get_ordered_keys(&first_keys, all_key_set);

    spew_headers(&ordered_keys);
    spew_records(&ordered_keys, &recs);

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
