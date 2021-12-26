use std::io;

pub fn run() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("where is the line");

    let start_fishes: Vec<usize> = line
        .trim()
        .split(",")
        .map(|x| x.parse().expect("NaN"))
        .collect();

    let mut fish_with_n_days_to_spawn: Vec<u64> = vec![0; 9];
    for fish in start_fishes {
        fish_with_n_days_to_spawn[fish] += 1;
    }
//    println!("after {} days: {:?}", 0, fish_with_n_days_to_spawn);
    
    let days = 256;
    for _day in 1..=days {
        let spawning = fish_with_n_days_to_spawn[0];
        for i in 0..8 {
            // each fish has one less day until it spawns
            fish_with_n_days_to_spawn[i] = fish_with_n_days_to_spawn[i + 1];
        }
        fish_with_n_days_to_spawn[6] += spawning;
        fish_with_n_days_to_spawn[8] = spawning;

//        println!("after {} days: {:?}", _day, fish_with_n_days_to_spawn);
    }

    let num_fishes: u64 = fish_with_n_days_to_spawn.iter().fold(0, |total, f| total + f); 
    println!("{} fishes after {} days", num_fishes, days);
}
