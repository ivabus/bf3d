
struct Pointer {
	x: usize,
	y: usize,
	z: usize,
}

pub fn bf3d(instructions: String, output_to_stdout: bool) -> Vec<char> {
	let mut field: Vec<Vec<Vec<u8>>> =
		vec![vec![vec![0u8; u8::MAX as usize]; u8::MAX as usize]; u8::MAX as usize]; // initialize data array
	let mut ip: usize = 0;								// ip stands for Instruction Pointer
	let mut dp: Pointer = Pointer {x: 0, y: 0, z: 0};	// dp stands for Data Pointer
	let mut output: Vec<char> = Vec::new();				// necessary for automated tests
	while ip != instructions.len() {
		if instructions.as_bytes()[ip] == b'>' && dp.x != 255 {
			dp.x += 1;
		} else if instructions.as_bytes()[ip] == b'<' && dp.x != 0 {
			dp.x -= 1;
		} else if instructions.as_bytes()[ip] == b'^' && dp.y != 255 {
			dp.y += 1;
		} else if instructions.as_bytes()[ip] == b'_' && dp.y != 0 {
			dp.y -= 1;
		} else if instructions.as_bytes()[ip] == b'/' && dp.z != 255 {
			dp.z += 1;
		} else if instructions.as_bytes()[ip] == b'\\' && dp.z != 0 {
			dp.z -= 1;
		} else if instructions.as_bytes()[ip] == b'+' {
			if field[dp.x][dp.y][dp.z] == 255 {
				field[dp.x][dp.y][dp.z] = 0;
			} else {
				field[dp.x][dp.y][dp.z] += 1;
			}
		} else if instructions.as_bytes()[ip] == b'-' {
			if field[dp.x][dp.y][dp.z] == 0 {
				field[dp.x][dp.y][dp.z] = 255;
			} else {
				field[dp.x][dp.y][dp.z] -= 1;
			}
		} else if instructions.as_bytes()[ip] == b'.' {
			let c = field[dp.x][dp.y][dp.z] as char;
			if output_to_stdout {
				print!("{}", c);
			} else {
				output.push(c);
			}
		} else if instructions.as_bytes()[ip] == b',' {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
			if input.as_bytes().len() > 0 {
				field[dp.x][dp.y][dp.z] = input.as_bytes()[0] as u8;
			} else {
				field[dp.x][dp.y][dp.z] = 0;
			}
		} else if instructions.as_bytes()[ip] == b'[' {
			if field[dp.x][dp.y][dp.z] == 0 {
				while instructions.as_bytes()[ip] != b']'{
					ip += 1;
					if ip == instructions.len() {
						println!("Syntax error");
						break;
					}
				}
			}
		} else if instructions.as_bytes()[ip] == b']' {
			if field[dp.x][dp.y][dp.z] != 0 {
				while instructions.as_bytes()[ip] != b'[' {
					ip -= 1;
					if ip == 0 && instructions.as_bytes()[ip] != b'[' {
						println!("Syntax error");
						break;
					}
				}
			}
		}
		ip += 1;
	}
	output
}
