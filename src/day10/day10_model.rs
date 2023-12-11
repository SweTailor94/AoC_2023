// model types for Day10
use std::collections::{HashMap, HashSet};


#[derive(Debug)]
pub struct PipeMap {
    map: HashMap<(i32, i32), char>,
    circuit: HashMap<(i32, i32), char>,
    start: (i32, i32),
    rows: i32,
    cols: i32,
    to_left: HashSet<(i32,i32)>,
    to_right: HashSet<(i32,i32)>,
}

impl PipeMap {
    pub fn new() -> Self {
        PipeMap {
            map: HashMap::new(),
            circuit: HashMap::new(),
            start: (0, 0),
            rows:0,
            cols:0,
            to_left: HashSet::new(),
            to_right: HashSet::new(),
        }
    }

    pub fn parse_lines(&mut self, lines: &Vec<String>) -> anyhow::Result<()> {
        self.rows = lines.len() as i32;
        self.cols = lines[0].len() as i32;
        lines.iter().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                match c {
                    'S' => {
                        self.start = (row as i32, col as i32);
                        self.map.insert((row as i32, col as i32), c);
                    }
                    '.' => { /* ignore, ground*/ }
                    c => { self.map.insert((row as i32, col as i32), c); }
                }
            })
        });
        Ok(())
    }

    pub fn solve_part_1(&mut self) -> usize {
        // find connections from start:
        let mut dir = self.find_first_possible_direction();
        self.circuit.insert(self.start,'S');
        let mut back = false;
        let mut steps = 0;
        let mut current_pos = self.start;
        while !back {
            current_pos = next_pos( dir, current_pos);
            if let Some(pipe) = self.map.get(&current_pos){
                self.circuit.insert(current_pos,*pipe);
                dir = next_direction(pipe,dir);
            } else {
                panic!("Something wrong with the map, no pipe at {:?} in step {}", current_pos, steps + 1 );
            }
            steps += 1;
            if current_pos == self.start{
                back = true;
            }
        };
        steps / 2
    }

    fn find_first_possible_direction(&mut self) -> Direction{
        let mut dir: Direction = Direction::None;
        // Up?
        let mut got_dir = match self.map.get(&up(self.start)) {
            None => { false }
            Some(c) => {
                if CAN_CONNECT_UP.contains(c) {
                    dir = Direction::Up;
                    true
                } else { false }
            }
        };
        // Down?
        if !got_dir {
            got_dir = match self.map.get(&down(self.start)) {
                None => { false }
                Some(c) => {
                    if CAN_CONNECT_DOWN.contains(c) {
                        dir = Direction::Down;
                        true
                    } else { false }
                }
            };
        }
        // Left
        if !got_dir {
            got_dir = match self.map.get(&left(self.start)) {
                None => { false }
                Some(c) => {
                    if CAN_CONNECT_LEFT.contains(c) {
                        dir = Direction::Left;
                        true
                    } else { false }
                }
            };
        }
        // Right
        if !got_dir {
            if !match self.map.get(&right(self.start)) {
                None => { false }
                Some(c) => {
                    if CAN_CONNECT_RIGHT.contains(c) {
                        dir = Direction::Right;
                        true
                    } else { false }
                }
            } {
                panic!("Can't go any direction!")
            }
        }
        dir
    }

    pub fn solve_part_2(&mut self) -> i32{
        // find one way to start the circuit
        let mut dir = self.find_first_possible_direction();
        println!("Starting in {:?}", dir);
        let mut back = false;
        let mut current = self.start;
        let mut pipe : char;
        while !back {
            current = next_pos(dir, current);
            pipe = *self.circuit.get(&current).unwrap();
            for p in pos_to_check_to_left(current, pipe, dir){
                if ! self.to_left.contains(&p) && ! self.circuit.contains_key(&p){
                    self.to_left.insert(p);
                }
            }
            for p in pos_to_check_to_right(current, pipe, dir){
                if !self.to_right.contains(&p) && !self.circuit.contains_key(&p){
                    self.to_right.insert(p);
                }
            }
            if current == self.start {
                back = true;
            } else {
                dir = next_direction(&pipe,dir);
            }
        }
        if self.to_right.intersection(&self.to_left).count() != 0{
            panic!("empty tiles on left of pipe and on right of pipe can never be same");
        }
        // find which set is
        // Since I printed the circuit of pipes i Know that the collections of empty tiles
        // that has at least one tile on the first row or first column will be outside the loop.
        // To make this work for all solutions you can try and fill empty spaces for both to_left and to_right
        // and then check which has any tile on any edge (e.g. row == 0)
        let inside =
        if self.to_left.iter().any(|(r,c)| *r==0 || *c==0 ){
            self.to_right.clone()
        } else {
            self.to_left.clone()
        };

        // Take each tile inside and fill empty spaces with tiles
        let mut all_empty_tiles : HashSet<(i32,i32)> = HashSet::new();
        for p in inside{
            if !all_empty_tiles.contains(&p){
                self.fill_empty_space(p,&mut all_empty_tiles);
            }
        }
        all_empty_tiles.len() as i32
    }

    fn fill_empty_space(&self, pos:(i32,i32), set: &mut HashSet<(i32, i32)>){
        if !set.insert(pos) {return;}
        let u = up(pos);
        if !self.circuit.contains_key(&u){
            self.fill_empty_space(u, set);
        }
        let d = down(pos);
        if !self.circuit.contains_key(&d){
            self.fill_empty_space(d, set);
        }
        let l = left(pos);
        if !self.circuit.contains_key(&l){
            self.fill_empty_space(l, set);
        }
        let r = right(pos);
        if !self.circuit.contains_key(&r){
            self.fill_empty_space(r, set);
        }
    }

    pub fn print_circuit(&self) -> (){
        println!("x = Left, o = Right");
        for row in 0..self.rows{
            for col in 0..self.cols{
                match self.circuit.get(&(row,col)){
                    None => {
                        if self.to_left.contains(&(row,col)){
                            print!("x");
                        } else if self.to_right.contains(&(row,col)) {
                            print!("o");
                        } else {
                            print!(" ");
                        }
                    }
                    Some(c) => {print!("{}",char_to_box(c))}
                }
            }
            println!();
        }
    }
}

fn pos_to_check_to_left(current: (i32,i32), c: char, dir:Direction) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    match c {
        '|' => { match dir {
            Direction::None => {}
            Direction::Up => {res.push(left(current));}
            Direction::Down => {res.push(right(current));}
            Direction::Left => {}
            Direction::Right => {}
        }},
        '-' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {res.push(down(current));}
            Direction::Right => {res.push(up(current));}
        }},
        'L' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {
                res.push(down(current));
                res.push(left(current));
            }
            Direction::Right => {}
        }},
        'J' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {
                res.push(right(current));
                res.push(down(current));
            }
            Direction::Left => {}
            Direction::Right => {}
        }},
        '7' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {}
            Direction::Right => {
                res.push(right(current));
                res.push(up(current));
            }
        }},
        'F' => { match dir {
            Direction::None => {}
            Direction::Up => {
                res.push(up(current));
                res.push(left(current));
            }
            Direction::Down => {}
            Direction::Left => {}
            Direction::Right => {}
        }},
        'S' =>{match dir {
            Direction::None => {}
            Direction::Up => {res.push(left(current));}
            Direction::Down => {res.push(right(current));}
            Direction::Left => {res.push(down(current));}
            Direction::Right => {res.push(up(current));}
        } },
        ch => panic!("Illegal character {}", ch ),
    };
    res
}
fn pos_to_check_to_right(current: (i32,i32),c: char, dir:Direction) -> Vec<(i32,i32)>{
    let mut res = Vec::new();
    match c {
        '|' => { match dir {
            Direction::None => {}
            Direction::Up => {res.push(right(current));}
            Direction::Down => {res.push(left(current));}
            Direction::Left => {}
            Direction::Right => {}
        }},
        '-' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {res.push(up(current));}
            Direction::Right => {res.push(down(current));}
        }},
        'L' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {
                res.push(left(current));
                res.push(down(current));
            }
            Direction::Left => {}
            Direction::Right => {}
        }},
        'J' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {}
            Direction::Right => {
                res.push(down(current));
                res.push(right(current));
            }
        }},
        '7' => { match dir {
            Direction::None => {}
            Direction::Up => {
                res.push(right(current));
                res.push(up(current));
            }
            Direction::Down => {}
            Direction::Left => {}
            Direction::Right => {}
        }},
        'F' => { match dir {
            Direction::None => {}
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {
                res.push(up(current));
                res.push(left(current));
            }
            Direction::Right => {}
        }},
        'S' =>{match dir {
            Direction::None => {}
            Direction::Up => {res.push(right(current));}
            Direction::Down => {res.push(left(current));}
            Direction::Left => {res.push(up(current));}
            Direction::Right => {res.push(down(current));}
        }},
        ch => panic!("Illegal character {}", ch ),
    };
    res
}
fn char_to_box(c:&char) -> char{
    match c{
        '|' =>  '\u{2502}',
        '-' =>  '\u{2500}',
        'L' =>  '\u{2514}',
        'J' =>  '\u{2518}',
        '7' =>  '\u{2510}',
        'F' =>  '\u{250C}',
        x => *x,
    }
}
fn next_pos(dir: Direction, current_pos: (i32, i32)) -> (i32,i32) {
    match dir {
        Direction::None => panic!("Must always have a direction!"),
        Direction::Up => up(current_pos),
        Direction::Down => down(current_pos),
        Direction::Left => left(current_pos),
        Direction::Right => right(current_pos),
    }
}
fn up((row, col): (i32, i32)) -> (i32, i32) {
    (row - 1, col)
}

fn down((row, col): (i32, i32)) -> (i32, i32) {
    (row + 1, col)
}

fn left((row, col): (i32, i32)) -> (i32, i32) {
    (row, col - 1)
}

fn right((row, col): (i32, i32)) -> (i32, i32) {
    (row, col + 1)
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

fn next_direction(pipe: &char, last_dir: Direction) -> Direction {
    match pipe {
        '|' => {
            match last_dir {
                Direction::Up => Direction::Up,
                Direction::Down => Direction::Down,
                _ => panic!("Entered | wrong way.")
            }
        }
        '-' => {
            match last_dir {
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
                _ => panic!("Entered - wrong way.")
            }
        }
        'L' => {
            match last_dir {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                _ => panic!("Entered L wrong way.")
            }
        }
        'J' => {
            match last_dir {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                _ => panic!("Entered J wrong way.")
            }
        }
        '7' => {
            match last_dir {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                _ => panic!("Entered J wrong way.")
            }
        }
        'F' => {
            match last_dir {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                _ => panic!("Entered J wrong way.")
            }
        }
        'S' => Direction::None,
        _ => { panic!("Ther's a hole in the pipe!") }
    }
}


// | is a vertical pipe connecting north and south.  '\u{2502}'
// - is a horizontal pipe connecting east and west.  '\u{2500}'
// L is a 90-degree bend connecting north and east.  '\u{2514}'
// J is a 90-degree bend connecting north and west.  '\u{2518}'
// 7 is a 90-degree bend connecting south and west.  '\u{2510}'
// F is a 90-degree bend connecting south and east.  '\u{250C}'
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
static CAN_CONNECT_UP: [char; 3] = ['|', '7', 'F'];
static CAN_CONNECT_DOWN: [char; 3] = ['|', 'L', 'J'];
static CAN_CONNECT_LEFT: [char; 3] = ['-', 'L', 'F'];
static CAN_CONNECT_RIGHT: [char; 3] = ['-', '7', 'J'];


// fn next_pos(tile:char, current:(u32,u32));


