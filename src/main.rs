use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Write, Read};

use bitvec::prelude::*;
use clap::{Arg, App};

const DEFAULT_LENGTH: u64 = 9;

fn main() -> io::Result<()> {
    let matches = App::new("unique-instr")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("LENGTH")
            .short("l")
            .long("length")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("LINE DELIM")
            .short("d")
            .long("inplace-delimiter")
            .takes_value(true)
            .required(false))
        .get_matches();

    let input_path =
        matches.value_of("INPUT").expect("No input asm program.");

    let line_len: u64 =
        matches.value_of("LENGTH")
            .map(|line_len| line_len.parse::<u64>())
            .unwrap_or(Ok(DEFAULT_LENGTH))
            .expect("Input LENGTH was not a valid u64");

    let line_delim: Option<u64> =
        matches.value_of("LINE DELIM")
            .map(|line_delim| {
                let line_delim = line_delim.parse::<u64>().expect("Input LINE DELIM not a valid u64");
                line_delim
            });

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

        if count == line_len {
            count = 0;
            handle.write_all("\n".as_bytes())?;
        }

        if let Some(line_delim) = line_delim {
            if (count) % line_delim == 0  && count > 0 {
                handle.write_all("_".as_bytes())?;
            }
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
