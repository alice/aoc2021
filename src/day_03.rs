use std::io;

pub fn run() {
    let mut diagnostic = Vec::<u32>::new();
    let size = read_diagnostic_from_stdin(&mut diagnostic);

    let pos = size - 1;
    let mut potential_oxygen_ratings = diagnostic.to_vec();
    let mask = 1 << pos;
    let filter = most_common_bit(pos, &potential_oxygen_ratings) << pos;
    let mut potential_carbon_ratings: Vec<u32> = potential_oxygen_ratings
        .drain_filter(|x| *x & mask != filter)
        .collect();

    println!(
        "after draining: potential_carbon_ratings {:?}, potential_oxygen_ratings: {:?}",
        potential_carbon_ratings, potential_oxygen_ratings
    );

    let oxygen_rating = find_rating(&mut potential_oxygen_ratings, pos, true);
    let carbon_rating = find_rating(&mut potential_carbon_ratings, pos, false);

    println!(
        "oxygen_rating: {:?}, carbon_rating: {:?}, product: {}",
        oxygen_rating,
        carbon_rating,
        oxygen_rating * carbon_rating
    );
}

fn find_rating(potential_ratings: &mut Vec<u32>, start_pos: usize, most: bool) -> u32 {
    let mut pos = start_pos;
    loop {
        pos -= 1;
        let mask = 1 << pos;
        let most_common = most_common_bit(pos, &potential_ratings);
        let bit = if most { most_common } else { most_common ^ 1 };
        let filter = bit << pos;
        println!(
            "pos: {}, most_common: {:b}, bit: {:b}, mask: {:b}, filter: {:b}",
            pos, most_common, bit, mask, filter
        );

        let _ = potential_ratings
            .drain_filter(|x| *x & mask != filter)
            .count();
        println!("after draining: potential_ratings {:?}", potential_ratings);

        if potential_ratings.len() == 1 {
            return potential_ratings[0];
        }
    }
}

fn most_common_bit(pos: usize, diagnostic: &Vec<u32>) -> u32 {
    let mut count = 0;
    let mask = 1 << pos;
    for binary in diagnostic {
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

    return if count >= 0 { 1 } else { 0 };
}

fn read_diagnostic_from_stdin(diagnostic: &mut Vec<u32>) -> usize {
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

    return size;
}
