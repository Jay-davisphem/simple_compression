extern crate flate2;

use flate2::{write::GzEncoder, Compression};
use std::{env::args, fs::File, io::copy, io::BufReader, time::Instant};

fn main() {
    let n = args().len();
    if n < 2 || n > 3 {
        eprintln!("Usage: `source` `<target>`, (if target is not given it be target.gz)");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output_file: String;
    if n == 2 {
        output_file = format!("{}.gz", args().nth(1).unwrap())
    } else {
        output_file = args().nth(2).unwrap()
    }
    let output = File::create(output_file).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "
    Source length: {:?}\n
    Target length: {:?}\n
    Elapsed: {:?}
    ",
        input.get_ref().metadata().unwrap().len(),
        output.metadata().unwrap().len(),
        start.elapsed()
    )
}
