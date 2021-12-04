use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
	let f = File::open("input.txt")?;
	let reader = BufReader::new(f);

	let mut old_num:  i32 = 0;
	let mut new_num:  i32;
	let mut increase: i32 = -1;

	for line in reader.lines() {
		new_num = line.unwrap().parse::<i32>().unwrap();
		if old_num < new_num {
			increase += 1;
		}
		old_num = new_num;
	}

	print!("{:?}\n", increase);
	Ok(())
}
