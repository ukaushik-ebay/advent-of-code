use std::env;

use day1::day1;
use day10::day10;
use day11::day11;
use day12::day12;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse().unwrap();
    println!("AOC DAY {day}");
    match day {
        1 => tokio::spawn(day1()).await.unwrap(),
        2 => tokio::spawn(day2()).await.unwrap(),
        3 => tokio::spawn(day3()).await.unwrap(),
        4 => tokio::spawn(day4()).await.unwrap(),
        5 => tokio::spawn(day5()).await.unwrap(),
        6 => tokio::spawn(day6()).await.unwrap(),
        7 => tokio::spawn(day7()).await.unwrap(),
        8 => tokio::spawn(day8()).await.unwrap(),
        9 => tokio::spawn(day9()).await.unwrap(),
        10 => tokio::spawn(day10()).await.unwrap(),
        11 => tokio::spawn(day11()).await.unwrap(),
        12 => tokio::spawn(day12()).await.unwrap(),
        _ => panic!("invalid day"),
    }
}
