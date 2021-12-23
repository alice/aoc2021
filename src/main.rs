#![feature(iter_advance_by)]

use std::env;

mod day_01;
mod day_02;

fn main() {
    // parse which day it is
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
	panic!("Expected day argument");
    }
    
    let day = &args[1];

    match day.as_ref() {
	"1" => day_01::run(),
	"2" => day_02::run(),
	_ => panic!("No day {}", day),
    }
}
