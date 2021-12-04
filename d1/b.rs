use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
	let f = File::open("input.txt")?;
	let reader = BufReader::new(f);

	let mut values: [i32; 4] = [0; 4];
	let mut index: usize     = 0;
	let mut increase: i32    = 0;

	for line in reader.lines() {
		values[index % 4] = line.unwrap().parse::<i32>().unwrap();
		// println!("{:?}: {:?}", index, values);
		if index >= 3 {
			// print!("comparing {:?} & {:?}", (index + 1) % 4, index % 4);
			if values[(index + 1) % 4] < values[index % 4] {
				// println!(": increase");
				increase += 1;
			}
			else {
				// println!(": nop", );
			}
		}
		index += 1;
	}

	println!("{:?}", increase);
	Ok(())
}
