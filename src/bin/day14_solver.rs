use advent_of_code_2023::input::parse_input_file;
use advent_of_code_2023::day14::day14_model::{ DishParser2};


fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = DishParser2::new();
    let _ = parse_input_file("src/day14/input.txt", &mut input);
    // part 1
     input.tilt_north();
     println!("Day 14 part 1 -> {}",input.calc_weight()); // input.print();

    println!("Start part2");
    let mut input = DishParser2::new();
    let _ = parse_input_file("src/day14/input.txt", &mut input);
    input.print_stat();


    println!("Day 14 part 2 -> {}",input.solve_part2());
      Ok(())
}




