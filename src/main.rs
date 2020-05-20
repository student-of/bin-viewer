use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Write, Read};

use bitvec::prelude::*;
use clap::{Arg, App};

fn main() -> io::Result<()> {
    let matches = App::new("unique-instr")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input_path =
        matches.value_of("INPUT").expect("No input asm program.");

    let input_path = PathBuf::from(input_path);
    
    let mut input_file = File::open(input_path)
        .expect("Unable to open input file");

    let mut buffer = Vec::new();
    let _total_read = input_file.read_to_end(&mut buffer).unwrap();
    let bit_buff: BitVec<Msb0, u8> = buffer.into();

    let std_out = io::stdout();
    let mut handle = std_out.lock();

    let mut count = 0;
    for bit in bit_buff.into_iter() {

        if count == 9 {
            count = 0;
            handle.write_all("\n".as_bytes())?;
        }

        count += 1;

        if bit {
            handle.write_all("1".as_bytes())?;
        } else {
            handle.write_all("0".as_bytes())?;
        }

    }

    Ok(())
}
