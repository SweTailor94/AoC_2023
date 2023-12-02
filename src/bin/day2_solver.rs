use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day2::day2_model::{parse_game,calc_part1, calc_part2};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day2/input.txt", parse_game);
    println!("Day 2 part 1: {}",calc_part1(&input));
    println!("Day 2 part 2: {}",calc_part2(&input));
      Ok(())
}

//fn parse_input_line(line:&str) -> usize{
//    line.len()
//}
