mod rational;
mod range_encoder;
mod range_serializer;

use crate::rational::Rational;
use range_encoder::RangeEncoder;
use range_serializer::RangeSerializer;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;


fn main() {
    if std::env::args().len() != 2{
        eprintln!("Usage: [file]");
    }else{
        eprintln!("Started");
        let file = std::env::args().nth(1).unwrap();
        let mut f = File::open(&file).unwrap();
        let re = RangeEncoder::initialize(&mut f).unwrap();
        eprintln!("Initialized");
        f.seek(SeekFrom::Start(0)).unwrap();
        let data = re.compress(&mut f).unwrap();
        eprintln!("Compressed");
        let mut cfile = File::create(format!("{}.rez", &file)).unwrap();
        RangeSerializer::write(re, data, &mut cfile).unwrap();
        eprintln!("Done");
    }

    /*
    let mut to_encode = vec![0u8,1u8,1u8,1u8, 2u8];

    let re = RangeEncoder::initialize(&mut &*to_encode).unwrap();
    let intermediate = re.compress(&mut &* to_encode).unwrap();

    let result = re.decompress(intermediate);

    println!("Before: {:?}", to_encode);
    println!("After: {:?}", result);*/
}
