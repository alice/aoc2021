use std::io;

pub fn run() {
    let mut lines: Vec<String> = io::read_to_string(&mut io::stdin())
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut num_1478s = 0;
    for line in lines {
        let outputs: Vec<String> = line
            .split(" | ")
            .last()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        for output in outputs {
            match output.len() {
                2 | 3 | 4 | 7 => num_1478s += 1,
                _ => {}
            }
        }
    }

    println!("num_1478s: {}", num_1478s);
}
