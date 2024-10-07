use std::env::args;
use std::fs::File;
use std::io::{BufReader, copy};
use std::time::Instant;
use flate2::Compression;
use flate2::write::GzEncoder;


fn main() {
    if args().len() != 3 {
        println!("usage : `source` `target`");
        return
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    println!("source len {:?}", input.get_ref().metadata().unwrap().len());
    println!("target len {:?}", output.metadata().unwrap().len());
    println!("elapsed {:?}", start.elapsed());
}