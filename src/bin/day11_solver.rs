use advent_of_code_2023::day11::day11_model::Space;
use advent_of_code_2023::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut the_space = Space::new();
    parse_input_file("src/day11/input.txt",&mut the_space)?;
    println!("Day 11 part 1 {}", the_space.calc_total_dist(1) );

    let mut the_larger_space = Space::new();
    parse_input_file("src/day11/input.txt",&mut the_larger_space)?;
    println!("Day 11 part 2 {}", the_larger_space.calc_total_dist(2));
      Ok(())
}


