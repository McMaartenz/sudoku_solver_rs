use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main () {
	let mut field: [[u8;9];9];
	match load_from_disk() {
		Err(why) => { println!("This program expects 'field.txt' in the same directory. Failed to load from disk: {}", why); return },
		Ok(new_field) => field = *new_field,
	}

	println!("Loaded");

	print_field(&field);
	if !solve(&mut field, 0, 0) {
		println!("Could not solve sudoku");
	} else {
		println!("Solved:");
		print_field(&field);
	}
}

fn has_duplicate(arr: &[u8;9]) -> bool {
	let mut set = HashSet::<u8>::new();
	for n in arr {
		if *n != 0 && !set.insert(*n) {
			return true;
		}
	}
	
	false
}

fn verify(field: &[[u8;9];9]) -> bool {
	/* Check rows */
	for row in field {
		if has_duplicate(row) {
			return false;
		}
	}

	/* Check columns */
	for coln in 0..9 {
		let mut col = [0u8;9];
		for n in 0..9 {
			col[n] = field[n][coln];
		}

		if has_duplicate(&col) {
			return false;
		}
	}

	/* Check blocks */
	for block_x in 0..3 {
		for block_y in 0..3 {
			let mut block = [0u8;9];

			/* For each block */
			for y in 0..3 {
				for x in 0..3 {
					block[y * 3 + x] = field[block_y * 3 + y][block_x * 3 + x];
				}
			}

			if has_duplicate(&block) {
				return false;
			}
		}
	}

	true
}

fn solve(field: &mut [[u8;9];9], mut x: usize, mut y: usize) -> bool {
	if x > 8 {
		x = 0;
		y += 1;
	}
	
	if y > 8 {
		return true; /* End of field */
	}

	if field[y][x] != 0 { /* Already filled */
		return solve(field, x + 1, y);
	}

	for n in 1..=9 {
		field[y][x] = n;

		if verify(field) {
			if solve(field, x + 1, y) {
				return true;
			}
		}
	}

	field[y][x] = 0; /* Backtrack */
	return false;
}

fn print_field(field: &[[u8;9];9]) {
	for row in 0..9 {
		for col in 0..9 {
			let n = field[row][col];
			print!("{}", if n == 0 { '-' } else { std::char::from_digit(n as u32, 10).unwrap() });

			if col % 3 == 2 {
				print!(" ");
			}
		}
		println!();

		if row % 3 == 2 {
			println!();
		}
	}
}

fn load_from_disk() -> Result<Box<[[u8;9];9]>, String> {
	let mut field = [[0u8;9];9];

	let path = Path::new("field.txt");
	let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => return Err(format!("Could not open {}: {}", display, why)),
		Ok(file) => file,
	};

	let mut string = String::new();
	match file.read_to_string(&mut string) {
		Err(why) => return Err(format!("Could not read {}: {}", display, why)),
		Ok(_) => {},
	};

	if string.lines().count() != 9 {
		return Err("Unexpected row size".to_string());
	}

	let mut y = 0;
	for line in string.lines() {
		if line.len() != 9 {
			return Err("Unexpected column size".to_string());
		}

		let mut x = 0;
		for n in line.chars() {
			field[y][x] = if n == '-' { 0 } else { n.to_digit(10).unwrap() as u8 };
			x += 1;
		}

		y += 1;
	}
	
	Ok(Box::new(field))
}
