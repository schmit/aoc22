use std::error::Error;
use std::str::FromStr;

mod topk;
mod day1;
mod day2;

#[derive(Debug)]
pub enum Part {
    A,
    B,
}

#[derive(Debug)]
pub struct Config {
    pub day: i32,
    pub part: Part,
    pub input_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item= String>) -> Result<Config, &'static str> {
        args.next();

        // Parse the day argument
        let day_str = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing day argument"),
        };

        let day = match i32::from_str(&day_str) {
            Ok(d) => d,
            Err(_) => return Err("Invalid day argument"),
        };

        // Parse the part argument
        let part_str = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing part argument"),
        };

        let part = match part_str.as_str() {
            "A" => Part::A,
            "B" => Part::B,
            _ => return Err("Invalid part argument"),
        };

        let input_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing input_path argument")
        };

        // Check if there are any extra arguments
        if args.next().is_some() {
            return Err("Extra arguments provided");
        };

        Ok(Config { day, part, input_path })
    }
}

pub fn run(config: Config, contents: &str) -> Result<(), Box<dyn Error>> {
    let result = match (config.day, &config.part) {
        (1, Part::A) => day1::day1_part_a(contents),
        (1, Part::B) => day1::day1_part_b(contents),
        (2, Part::A) => day2::day2_part_a(contents),
        (2, Part::B) => day2::day2_part_b(contents),

        _ => -1,
    };

    println!("Answer: {result}");

    Ok(())
}