use std::io;
use std::io::Read;

pub fn run() {
    // read input file from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let depths: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse().expect(
                format!("could not parse {:?}", x).as_str(),
            )
        })
	.collect();

    let mut iter = depths.iter().peekable();
    let mut increases = 0;
    loop {
	let curr = iter.next();
	let next = iter.peek();
	if next == None {
	    break;
	}

	if *next.unwrap() > curr.unwrap() {
	    increases += 1;
	}
    }

    println!("increases: {}", increases);
}
