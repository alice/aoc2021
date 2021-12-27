#![feature(iter_advance_by)]
#![feature(drain_filter)]
#![feature(io_read_to_string)]

use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

fn main() {
    // parse which day it is
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        day_08::run();
        return;
    }
    
    let day = &args[1];

    match day.as_ref() {
	"1" => day_01::run(),
	"2" => day_02::run(),
        "3" => day_03::run(),
        "4" => day_04::run(),
        "5" => day_05::run(),
        "6" => day_06::run(),
        "7" => day_07::run(),
        "8" => day_08::run(),
        _ => panic!("that's not a day"),
    }
}
