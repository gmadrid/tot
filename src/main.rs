use std::collections::HashSet;

fn main() {
    let f = std::fs::File::open("src/testinput/one.txt").unwrap();

    let tot = tot::Tot::read_from(f).unwrap();
    let recs = tot.take_records().unwrap();

    let mut all_keys = HashSet::<&str>::default();
    for rec in &recs {
        all_keys.extend(rec.keys().map(|s| s.as_str()));
    }

    let mut sep = "";
    for key in all_keys.iter() {
        print!("{}{}", sep, key);
        sep = "\t";
    }
    println!();

    for rec in &recs {
        sep = "";
        for key in all_keys.iter() {
            if let Some(value) = rec.get(*key) {
                print!("{}{}", sep, value);
            } else {
                print!("{}", sep)
            }

            sep = "\t";
        }
        println!();
    }
}
