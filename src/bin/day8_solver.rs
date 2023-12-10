use advent_of_code_2023::day8::day8_model::DessertMap;
use advent_of_code_2023::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = DessertMap::new();
    parse_input_file("src/day8/input.txt", &mut input).unwrap();
    println!("Day 8 part 1 -> {}", input.follow_directions("AAA".to_string(),"ZZZ".to_string()));


    println!("Start {:?}", input.ends_w_a);
    println!("Goal {:?}", input.ends_w_z);
    println!("Day 8 part 2 -> {}", input.part_2());
    Ok(())
}


