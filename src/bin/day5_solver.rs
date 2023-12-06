use advent_of_code_2023::input::parse_input_file;
use advent_of_code_2023::day5::day5_model::{SeedMap};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = SeedMap::new();
    parse_input_file("src/day5/input.txt", &mut input ).unwrap();
    println!("Day 5 part 1 -> {}",input.calc_part_1());
    println!("Day 5 part 2 -> {}",input.calc_part_2());
      Ok(())
}
//Day 5 part 1 -> 462648396
//Day 5 part 2 -> 2520479

//fn parse_input_line(line:&str) -> usize{    line.len()}
