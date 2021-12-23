#![feature(iter_advance_by)]
#![feature(drain_filter)]

use std::env;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    // parse which day it is
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        day_03::run();
        return;
    }
    
    let day = &args[1];

    match day.as_ref() {
	"1" => day_01::run(),
	"2" => day_02::run(),
        "3" | _ => day_03::run(),
    }
}
