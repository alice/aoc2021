use std::io;
use std::str::FromStr;
use std::string::ParseError;

struct Instruction {
    pub dir: String,
    pub dist: i32,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	if let [dir, dist] = s.split_whitespace().collect::<Vec<&str>>().as_slice() {
	    return Ok(Instruction {
		dir: dir.to_string(),
		dist: dist.parse().unwrap(),
	    });
	}
	panic!("Incorrect input format: '{}'", s);
    }
}

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
}

pub fn run() {
    let mut location  = Location{x: 0, y: 0};

    loop {
	let mut line = String::new();
	match io::stdin().read_line(&mut line) {
	    Ok(0) => {
		break;
	    }
	    Ok(_) => {
		// parse instruction
		let instruction: Instruction = line.parse().unwrap();
		match instruction.dir.as_str() {
		    "forward" => { location.x += instruction.dist },
		    "up" => { location.y -= instruction.dist },
		    "down" => { location.y += instruction.dist },
		    &_ => {},
		};
	    }
	    Err(err) => {
		panic!("{}", err);
	    }
	}
    }

    println!("{:?} -> {}", location, location.x * location.y);
}
