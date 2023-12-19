use advent_of_code_2023::day13::day13_model::PatternParser;
use advent_of_code_2023::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut mirrors =  PatternParser::new();
    parse_input_file("src/day13/input.txt", &mut mirrors);
    println!("Day 13 part 1 -> {}", mirrors.solve_p1());
    println!("Day 13 part 2 -> {}", mirrors.solve_p2());
      Ok(())
}

//fn parse_input_line(line:&str) -> usize{
//    line.len()
//}
