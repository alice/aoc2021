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

    let mut iter1 = depths.iter().peekable();
    let mut iter2 = depths.iter().peekable();
    if iter2.advance_by(1) != Ok(()) {
	panic!("Not enough input values");
    }
    let mut iter3 = depths.iter().peekable();
    if iter3.advance_by(2) != Ok(()) {
	panic!("Not enough input values");
    }

    let mut increases = 0;
    loop {
	let curr1 = iter1.next().unwrap();
	let next1 = iter1.peek().unwrap();

	let curr2 = iter2.next().unwrap();
	let next2 = iter2.peek().unwrap();

	let curr3 = iter3.next().unwrap();
	let maybe_next3 = iter3.peek();

	if maybe_next3 == None {
	    break;
	}
	let next3 = maybe_next3.unwrap();

        let curr = curr1 + curr2 + curr3;
	let next = *next1 + *next2 + *next3;

	if next > curr {
	    increases += 1;
	}
    }

    println!("increases: {}", increases);
}
