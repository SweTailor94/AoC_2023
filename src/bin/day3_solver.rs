use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day3::day3_model::{parse_line, part1, part2};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day3/input.txt", parse_line);
    println!("Day 3 part 1 -> {}",part1(&input));
    println!("Day 3 part 2 -> {}", part2(&input));
      Ok(())
}


// fn parse_input_line(line:&str) -> usize{   line.len() }
