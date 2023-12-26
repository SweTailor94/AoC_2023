use advent_of_code_2023::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day21/input.txt", parse_input_line);
    println!("Day 21 part 1 ");
    println!("Day 21 part 2 ");
      Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
