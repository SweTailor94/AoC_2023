use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day4::day4_model::{parse_card, calc_points_part1, calc_cards_part2};
fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day4/input.txt", parse_card);
    println!("Day 4 part 1 -> {}",calc_points_part1(&input));
    println!("Day 4 part 2 -> {}", calc_cards_part2(&input));
      Ok(())
}

// fn parse_input_line(line:&str) -> usize{    line.len()}
