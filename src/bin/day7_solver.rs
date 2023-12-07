use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day7::day7_model::{Hand, Hand2,calc_part_1,calc_part_2};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day7/input.txt", parse_input_line)    ;
    println!("Day 7 part 1 -> {}", calc_part_1(&input));
    let  input = get_vector_from_file("src/day7/input.txt", parse_input_line_part2)    ;
    println!("Day 7 part 2 -> {}", calc_part_2(&input));
      Ok(())
}

fn parse_input_line(line:&str) -> Hand{
    Hand::new(line)
}

fn parse_input_line_part2(line:&str) -> Hand2{
    Hand2::new(line)
}