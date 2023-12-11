use advent_of_code_2023::day10::day10_model::PipeMap;
use advent_of_code_2023::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day10/input.txt", parse_input_line);
    let mut the_map = PipeMap::new();
    the_map.parse_lines(&input)?;

    println!("Day 10 part 1 -> {}",the_map.solve_part_1());
    println!("Day 10 part 2 -> {}",the_map.solve_part_2());
    the_map.print_circuit();
    Ok(())
}

fn parse_input_line(line:&str) -> String{
    line.to_string()
}
