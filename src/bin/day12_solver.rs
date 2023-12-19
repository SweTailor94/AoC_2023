use itertools::Itertools;
use advent_of_code_2023::input::get_vector_from_file;
use advent_of_code_2023::day12::day12_model::valid_combinations;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day12/input.txt", parse_input_line);
    println!("Day 12 part 1 {}",input.iter().map(|(s,target)|valid_combinations(s,target)).sum::<u32>());
    println!("Day 12 part 2 ");
      Ok(())
}


fn parse_input_line(line:&str) -> (String,Vec<u32>){
    let parts:Vec<String> = line.split(' ').map(|x|x.trim().to_string()).collect();
    let numeric_groups = parts[1].split(',').map(|x|x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    (parts[0].clone(),numeric_groups)
}

#[cfg(test)]
mod test{
    use advent_of_code_2023::day12::day12_model::valid_combinations;
    use crate::parse_input_line;

    #[test]
    fn test_1(){
        let inp = ".?#????#????????#?# 1,1,1,1,1,6";
        let a = parse_input_line(inp);
        println!("{}", valid_combinations(&a.0, &a.1));
    }
}