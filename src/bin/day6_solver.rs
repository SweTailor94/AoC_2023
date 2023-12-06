

fn main() ->Result<(),Box<dyn std::error::Error>> {

    let input = vec![   (51,222), (92, 2031), (68 ,1126), (90,1225)];
    let part1: i64 = input.iter().map(calc_winning_possibilities).product();
    let part2: i64 = calc_winning_possibilities(&(51926890,222203111261225));
    println!("Day 6 part 1 -> {}",part1);
    println!("Day 6 part 2 -> {}",part2);
      Ok(())
}

fn calc_winning_possibilities(race: &(i64,i64)) -> i64{
    let (time, dist) = race;
    // d = pt * (t-pt) > dist
    // pt*t - pt^2 = dist
    // 0 = pt^2 - pt*t + dist
    // pt = t/2 +- sqrt( t^2/4 -dist)
    let t = *time as f64;
    let d = *dist    as f64;
    let pt1 = t/2.0 - (t*t/4.0 - d).sqrt();
    let pt2 = t/2.0 + (t*t/4.0 - d).sqrt();
    pt2 as i64 - pt1 as i64
}

//fn parse_input_line(line:&str) -> usize{    line.len()}
