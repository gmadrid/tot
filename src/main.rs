use argh::FromArgs;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::iter::Extend;
use std::path::PathBuf;
use tot::{Record, Records};

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

    #[argh(option, long = "output", short = 'o')]
    /// output file. If not provided, output to stdout.
    output_filename: Option<PathBuf>,

    #[argh(positional)]
    /// the file to process. (Currently only a single file is allowed.)
    filename: Option<PathBuf>,
}

type KeySet<'a> = HashSet<&'a str>;

fn get_input_records(instream: impl BufRead, input_separator: &str, trim: bool) -> tot::Result<Records> {
    tot::read_records_from_chunks(instream, input_separator, trim)
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

fn spew_headers(outstream: &mut impl Write, keys: &[&str], separator: &str) -> tot::Result<()> {
    writeln!(outstream, "{}", keys.iter().join(separator))?;
    Ok(())
}

fn spew_records(
    outstream: &mut impl Write,
    keys: &[&str],
    records: &[Record],
    separator: &str,
) -> tot::Result<()> {
    for rec in records {
        writeln!(
            outstream,
            "{}",
            keys.iter()
                .map(|k| { rec.get(*k).map(|v| v.as_str()).unwrap_or("") })
                .join(separator)
        )?;
    }
    Ok(())
}

fn get_instream(filename: &Option<PathBuf>) -> tot::Result<Box<dyn BufRead>> {
    Ok(match filename {
        None => Box::new(BufReader::new(io::stdin())),
        Some(name) => Box::new(BufReader::new(File::open(name)?)),
    })
}

fn get_outstream(filename: &Option<PathBuf>) -> tot::Result<Box<dyn io::Write>> {
    Ok(match filename {
        None => Box::new(io::stdout()),
        Some(name) => Box::new(File::create(name)?),
    })
}

fn process() -> tot::Result<()> {
    let args = argh::from_env::<Args>();

    let instream = get_instream(&args.filename)?;
    let recs = get_input_records(instream, &args.input_separator, args.trim)?;
    let all_key_set = get_all_key_names(&recs);
    let first_keys = get_first_keys(&args.order);
    let ordered_keys = get_ordered_keys(&first_keys, all_key_set);

    let mut outstream = get_outstream(&args.output_filename)?;

    spew_headers(&mut outstream, &ordered_keys, &args.output_separator)?;
    spew_records(&mut outstream, &ordered_keys, &recs, &args.output_separator)?;

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
