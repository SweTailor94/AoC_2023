use itertools::Itertools;
use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day15::day15_model::{hash,LensArray};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let solution1 = get_vector_from_file("src/day15/input.txt", parse_input_line);
    println!("Day 15 part 1 {}" , solution1[0]);
    let input2 = get_vector_from_file("src/day15/input.txt",parse_input_part2);
    let mut lenses = LensArray::new();
    input2[0].iter().for_each(|instr|lenses.add_rule(instr));
    let sum = lenses.calc_val_part2();
    println!("Day 15 part 2 -> {}",sum);
      Ok(())
}

fn parse_input_line(line:&str) -> u64{
    line.split(',').map(|x|hash(x) as u64).sum()
}

fn parse_input_part2(line:&str) ->Vec<String> {
    line.split(',').map(|m|m.to_string()).collect_vec()
}