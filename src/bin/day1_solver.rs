use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day1::day1_model::{parse_calibration,parse_calibration_2};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day1/input.txt", parse_input_line);
    let sum:u32 =  input.iter().sum();
    println!("Day 1 part 1. Sum {}",sum);
    let input = get_vector_from_file("src/day1/input.txt", parse_calibration_2);
    let sum:u32 =  input.iter().sum();
    println!("Day 1 part 2. Sum {}",sum);
    Ok(())
}

fn parse_input_line(line:&str) -> u32{

    parse_calibration(line)
}
