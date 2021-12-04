use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
	let f = File::open("input.txt")?;
	let reader = BufReader::new(f);

	let mut amount_of_lines: u32     = 0;
	let mut amount_of_ones: Vec<u32> = vec![];

	for line in reader.lines() {

		let str = line?;

		let char_count = str.chars().count();
		if char_count > amount_of_ones.len() {
			amount_of_ones.resize(char_count, 0);
		}
		for (pos, char) in str.chars().enumerate() {
			if char == '1' {
				amount_of_ones[pos] += 1;
			}
		}
		amount_of_lines += 1;
	}

	let mut gamma: u64 = 0;
	for num in amount_of_ones.iter() {
		gamma <<= 1;
		if num * 2 > amount_of_lines {
			gamma += 1;
		}
	}

	let epsilon = !gamma & !(!0 << amount_of_ones.len() - 1);
	println!("gamma   : {:?}", gamma);
	println!("epsilon : {:?}", epsilon);
	println!("product : {:?}", gamma * epsilon);

	let read_again = BufReader::new(f);

	let mut matching_ox_gen: &str = ""
	let mut score_ox_gen:    u8   = 0

	for line in read_again.lines() {

		let str = line?;
		if 
	}
	Ok(())
}
