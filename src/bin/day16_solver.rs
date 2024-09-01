use advent_of_code_2023::day16::day16_model::Contraption;
use advent_of_code_2023::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = Contraption::new();

    let _ = parse_input_file("src/day16/input.txt", &mut input);
    input.print_stats();
    println!("Day 16 part 1 -> {}",input.run_beam_get_energized_tiles());
    println!("Day 16 part 2 -> {}",input.find_maximum());
      Ok(())
}

