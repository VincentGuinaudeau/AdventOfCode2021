use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

enum Direction {
	Forward,
	Down,
	Up,
}

fn parse_line(str: String) -> (Direction, u32) {
	let mut split = str.split_whitespace();

	return (
		match split.next() {
			Some("forward") => Direction::Forward,
			Some("down")    => Direction::Down,
			Some("up")      => Direction::Up,
			Some(rest)      => panic!("invalid direction value '{:?}'", rest),
			None            => panic!("expected value")
		},
		match split.next() {
			Some(word) => match word.parse::<u32>() {
				Ok(num)   => num,
				Err(_err) => panic!("invalid value number '{:?}'", word),
			},
			None => panic!("expected value after direction"),
		},
	);
}

fn main() -> io::Result<()> {
	let f = File::open("input.txt")?;
	let reader = BufReader::new(f);

	let mut distance: u32 = 0;
	let mut depth:    u32 = 0;

	for line in reader.lines() {
		match parse_line(line?) {
			(Direction::Forward, num) => distance += num,
			(Direction::Down,    num) => depth    += num,
			(Direction::Up,      num) => depth    -= num,
		}
	}

	print!("distance : {:?}\n", distance);
	print!("depth : {:?}\n", depth);
	print!("product : {:?}\n", distance * depth);
	Ok(())
}
