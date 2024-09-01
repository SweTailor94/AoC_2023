use std::collections::{ HashSet, VecDeque};
use advent_of_code_2023::input::get_vector_from_file;
use hex::FromHex;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 18 part 1 ");
    let input = get_vector_from_file("src/day18/input.txt", parse_input_line);
    let mut pool = Pool::new();
    pool.create_perimeter(input);
    // pool.print();
    println!("Total volume {} m3", pool.calc_total());
    println!("Day 18 part 2 ");
    let input = get_vector_from_file("src/day18/input.txt", parse_input_line2);
    println!("Got {} digg instructions",input.len());
    let mut pool = Pool::new();
    pool.create_perimeter(input);
    print!("Calculating ...");
    println!("Total volume {} m3", pool.calc_total());
    Ok(())
}

struct Pool {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    perimeter: HashSet<(i64,i64)>,
    largest_run: u32,
    inside: (i64, i64),
}


impl Pool {
    pub fn new() -> Self {
        Pool {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            perimeter: Default::default(),
            largest_run: 0,
            inside: (0, 0),
        }
    }
    pub fn calc_total(&mut self) -> usize {

        println!("Perimeter is {} m3", self.perimeter.len());

        println!("Start exploring at {:?}", self.inside);
        Pool::all_inside(&mut self.perimeter, self.inside);
        self.perimeter.len()
    }


    fn all_inside(world: &mut HashSet<(i64,i64)>, start: (i64, i64))  {
        let (x, y) = start;
        if let Some(_) = world.get(&(y , x)) {
            panic!("Must start on empty spot.");
        } else {
            let mut possible: VecDeque<(i64, i64)> = VecDeque::new();
            possible.push_back((y ,  x ));

            while let Some(pos) = possible.pop_front() {
                if let Some(_) = world.get(&pos) {
                    continue;
                } else {
                    let (y,x) = pos;
                    world.insert(pos);
                    possible.push_back((y, x - 1));
                    possible.push_back((y, x + 1));
                    possible.push_back((y - 1, x));
                    possible.push_back((y + 1, x));
                }
            }
        }
    }
    pub fn create_perimeter(&mut self, diggs: Vec<DiggInstruction>) -> () {
        // Start att (0,0)
        let mut cur = (0, 0);
        self.perimeter.insert((0, 0));
        let mut right_turns = 0u32;
        let mut left_turns = 0u32;
        let mut last_direction = Direction::Start;
        let mut last_step: &DiggInstruction = &diggs[0];
        for step in &diggs {
            match last_direction {
                Direction::Up => {
                    match step.direction {
                        Direction::Up |
                        Direction::Down |
                        Direction::Start => {} // No turn or illegal
                        Direction::Right => right_turns += 1,
                        Direction::Left => left_turns += 1,
                    }
                }
                Direction::Down => {
                    match step.direction {
                        Direction::Up |
                        Direction::Down |
                        Direction::Start => {} // No turn or illegal
                        Direction::Right => left_turns += 1,
                        Direction::Left => right_turns += 1,
                    }
                }
                Direction::Right => {
                    match step.direction {
                        Direction::Up => left_turns += 1,
                        Direction::Down => right_turns += 1,
                        Direction::Right |
                        Direction::Left |
                        Direction::Start => {} // No turn or illegal
                    }
                }
                Direction::Left => {
                    match step.direction {
                        Direction::Up => right_turns += 1,
                        Direction::Down => left_turns += 1,
                        Direction::Right |
                        Direction::Left |
                        Direction::Start => {} // No turn or illegal
                    }
                }
                Direction::Start => {} // first digg, no turn.
            }
            last_direction = step.direction.clone();
            last_step = step;
            let (dx, dy) = step.steps;
            self.largest_run = self.largest_run.max(dx.max(dy) as u32);
            cur = self.add_steps(step.steps, cur);
            println!("Steps so far {}", self.perimeter.len());
        }

        if cur != (0, 0) { panic!("We should be back where we started? (0,0) but we are here {:?}", cur) }

        //find a point inside the perimeter
        println!("Right turns {}, Left turns {}", right_turns, left_turns);
        println!("Last step {:#?}", last_step);
        self.inside = if right_turns > left_turns { // the loop i clockwise (in total)
            match last_step.direction {
                Direction::Up => (1, -1),
                Direction::Down => (-1, 1),
                Direction::Right => (-1, -1),
                Direction::Left => (1, 1),
                Direction::Start => { unreachable!("There has to be a direction here") }
            }
        } else {
            match last_step.direction {
                Direction::Up => (-1, -1),
                Direction::Down => (1, 1),
                Direction::Right => (-1, 1),
                Direction::Left => (1, -1),
                Direction::Start => unreachable!("There has to be a direction here"),
            }
        };

        println!("The world :");
        println!("  {} <= x <= {}", self.x_min, self.x_max);
        println!("  {} <= y <= {}", self.y_min, self.y_max);
        println!("  largets run {}", self.largest_run);
        println!("  inside {:?}", self.inside);
    }


    pub fn add_steps(&mut self, step: (i64, i64), cur: (i64, i64)) -> (i64, i64) {
        let (d_x, d_y) = step;
        if !(d_x == 0 && d_y != 0 || d_x != 0 && d_y == 0) { panic!("Shall move only in one direction! {:?}", step) }
        let (mut cur_x, mut cur_y) = cur;

        let inc_dec = if d_x < 0 { -1 } else if d_x > 0 { 1 } else { 0 };
        if inc_dec != 0 {
            for _ in 0..d_x.abs()
            {
                cur_x += inc_dec;
                self.perimeter.insert ((cur_y,cur_x) );
            }
        }
        let inc_dec = if d_y < 0 { -1 } else if d_y > 0 { 1 } else { 0 };
        if inc_dec != 0 {
            for _ in 0..d_y.abs()
            {
                cur_y += inc_dec;
                self.perimeter.insert((cur_y , cur_x) );
            }
        }
        self.check_min_max(cur_x,cur_y);
        (cur_x, cur_y)
    }
    fn check_min_max(&mut self, x: i64, y: i64) -> () {
        self.x_min = x.min(self.x_min);
        self.x_max = x.max(self.x_max);
        self.y_min = y.min(self.y_min);
        self.y_max = y.max(self.y_max);
    }

}


#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Start,
}

#[derive(Clone, Debug)]
struct DiggInstruction {
    pub direction: Direction,
    pub steps: (i64, i64), // (x,y) delta

}

fn parse_input_line(line: &str) -> DiggInstruction {
    // R 6 (#3a8da2)

    let line_parts: Vec<&str> = line.split(' ').collect();

    let steps: i64 = match line_parts[1].parse() {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let (direction, steps) = match line_parts[0] {
        "U" => (Direction::Up, (0, steps)),
        "D" => (Direction::Down, (0, -steps)),
        "R" => (Direction::Right, (steps, 0)),
        "L" => (Direction::Left, (-steps, 0)),
        _ => panic!("Unknown direction in input {}", line_parts[0]),
    };


    DiggInstruction {
        direction,
        steps,
    }
}
fn parse_input_line2(line: &str) -> DiggInstruction {
    // R 6 (#3a8da2)

    let line_parts: Vec<&str> = line.split(' ').collect();

    let mut multi = 0x10000;
    let mut colour = 0u32;
    let buff:String = line_parts[2][2..8].to_string();
    for c in <Vec<u8>>::from_hex(buff).unwrap() {
        colour += (c as i32 * multi) as u32;
        multi = multi >> 8;
    };

    let steps = (colour >> 4) as i64;


    let (direction, steps) = match &line_parts[2][7..8] {
        "3" => (Direction::Up, (0, steps)),
        "1" => (Direction::Down, (0, -steps)),
        "0" => (Direction::Right, (steps, 0)),
        "2" => (Direction::Left, (-steps, 0)),
        _ => panic!("Unknown direction in input {}", line_parts[0]),
    };

    DiggInstruction {
        direction,
        steps,
    }
}