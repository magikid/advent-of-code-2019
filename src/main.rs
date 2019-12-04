use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day.as_str() {
        "1" => println!("day1 p1: {:?}, p2: {:?}", day1::p1(), day1::p2()),
        "2" => println!("day2 p1: {:?}, p2: {:?}", day2::p1(), day2::p2()),
        "3" => println!("day3 p1: {:?}, p2: {:?}", day3::p1(), day3::p2()),
        "4" => println!("day4 p1: {:?}, p2: {:?}", day4::p1(), day4::p2()),
        _ => panic!("You forgot to specify a day"),
    }
}
