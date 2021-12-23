use std::io;

pub fn run() {
    let mut diagnostic = Vec::<u32>::new();
    let mut size: usize = 0;

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                line = line.trim().to_string();
                if size == 0 {
                    size = line.len();
                }
                println!("line: '{}'", line);
                let binary = u32::from_str_radix(&line, 2).unwrap();
                diagnostic.push(binary);
            }
            Err(_) => {
                panic!("bad line: {}", line);
            }
        }
    }

    let mut potential_oxygen_ratings = diagnostic.to_vec();

    let mut pos = size;
    loop {
        pos -= 1;
        let mut count = 0;
        let mask = 1 << pos;
        for binary in &potential_oxygen_ratings {
            let bit = (*binary & mask) / mask;
            match bit {
                0 => {
                    count -= 1;
                }
                1 => {
                    count += 1;
                }
                _ => {}
            }
        }

        let filter = (if count >= 0 { 1 } else { 0 }) << pos;
        let _ = potential_oxygen_ratings
            .drain_filter(|x| *x & mask != filter)
            .count();
        println!("after draining: {:?}", potential_oxygen_ratings);

        if potential_oxygen_ratings.len() == 1 {
            break;
        }
    }
    let oxygen_rating = potential_oxygen_ratings[0];

    let mut potential_carbon_ratings = diagnostic.to_vec();
    let mut pos = size;
    loop {
        pos -= 1;
        let mut count = 0;
        let mask = 1 << pos;
        for binary in &potential_carbon_ratings {
            let bit = (*binary & mask) / mask;
            match bit {
                1 => {
                    count -= 1;
                }
                0 => {
                    count += 1;
                }
                _ => {}
            }
        }

        let filter = (if count > 0 { 1 } else { 0 }) << pos;
        let _ = potential_carbon_ratings
            .drain_filter(|x| *x & mask != filter)
            .count();
        println!("after draining: {:?}", potential_carbon_ratings);

        if potential_carbon_ratings.len() == 1 {
            break;
        }
    }
    let carbon_rating = potential_carbon_ratings[0];

    println!(
        "oxygen_rating: {:?}, carbon_rating: {:?}, product: {}",
        oxygen_rating,
        carbon_rating,
        oxygen_rating * carbon_rating
    );
}
