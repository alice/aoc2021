use std::io;

pub fn run() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("where is the line");

    let mut locations: Vec<i32> = line.trim().split(",").map(|x| x.parse().unwrap()).collect();
    locations.sort();

    let median = locations[locations.len() / 2];
    println!("median: {}", median);

    let total_fuel_linear: i32 = locations
        .iter()
        .fold(0, |total, l| total + (*l - median).abs());

    println!("total fuel linear: {}", total_fuel_linear);

    let max_location: i32 = *locations.last().unwrap();
    let mut total_for_location = vec![0; max_location as usize + 1];
    for l in 0..=max_location {
        for m in &locations {
            let raw_distance = (l - m).abs();
            let fuel = (raw_distance * (raw_distance + 1))/2;
            total_for_location[l as usize] += fuel;
        }
    }

    let mut min = i32::MAX;
    let mut min_loc = 0;
    for i in 0..locations.len() {
        let fuel = total_for_location[i];
        if fuel < min {
            min = fuel;
            min_loc = i;
       }
    }

    println!("total fuel fancy: {} at {}", min, min_loc);
}
