use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day9::day9_model::{part1,part2};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day9/input.txt", parse_input_line);

    println!("Day 9 part 1 {:}", part1(&input) );
    println!("Day 9 part 2 {:}", part2(&input) );
      Ok(())
}

fn parse_input_line(line:&str) -> Vec<i64>{
    line.split(' ').map(|x|x.parse::<i64>().unwrap()).collect()
}
