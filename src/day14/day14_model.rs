// model types for Day14


use std::collections::{HashMap};
use crate::input::InputParser;

#[derive(PartialEq)]
pub enum Rock {
    Round,
    // can move to empty spot
    Square,
    // Never moves
    Empty, // empty space where round can move in to
}


pub struct DishParser {
    pub round_rocks: HashMap<usize, Vec<usize>>,
    // Key == column, Vec is rows
    pub square_rocks: HashMap<usize, Vec<usize>>,
    // Key == column, Vec is rows
    pub row: usize,
    pub columns: usize,
}

impl DishParser {
    pub fn new() -> Self {
        DishParser {
            round_rocks: Default::default(),
            square_rocks: Default::default(),
            row: 0,
            columns: 0,
        }
    }

    pub fn tilt(&mut self) -> () {
        for i in 0..self.columns {
            if let Some(round) = self.round_rocks.get_mut(&i) {
                match self.square_rocks.get(&i) {
                    None => {
                        // No square rocks at all in this column,
                        // move all round to the north (small indices)
                        for i in 0..round.len() {
                            round[i] = i;
                        };
                    }
                    Some(square) => {
                        let mut stop: i32 = -1;
                        for r_i in 0..round.len() {
                            if let Some(i) = get_closest_square(square, round[r_i]) {
                                if *i as i32 >= stop {
                                    round[r_i] = i + 1;
                                    stop = *i as i32 + 1;
                                } else {
                                    round[r_i] = (stop + 1) as usize;
                                    stop += 1;
                                }
                            } else {
                                // there are no square rocks between this round rock and the ´north most position
                                // put this after the last one
                                round[r_i] = (stop + 1) as usize;
                                stop += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn calc_wheight_part1(&self) -> u64 {
        let mut sum = 0;
        for (_c, col) in self.round_rocks.iter() {
            // print!("Col {} =>",_c );
            for r in col {
                let weight = (self.row as u64) - 1 - (*r as u64);
                // print!("{} ", weight);
                sum += weight;
            }
            // println!("  Sum: {}",sum);
        }
        sum
    }

    pub fn print_columns(&self) {
        for i in 0..self.columns {
            print!("Round rocks in column {}", i);
            match self.round_rocks.get(&i) {
                None => { println!() }
                Some(rows) => { println!("{:?}", rows) }
            }
        }
    }

    pub fn print(&self) {
        let mut sum = 0u64;
        for row in 0..self.row {
            for col in 0..self.columns {
                let mut c = match self.round_rocks.get(&col) {
                    None => '.',
                    Some(v) =>
                        if v.contains(&row) {
                            sum = sum + (self.row as u64) - 1 - (row as u64);
                            'O'
                        } else { '.' }
                };
                match self.square_rocks.get(&col) {
                    None => {}
                    Some(v) => if v.contains(&row) {
                        if c == '.' {
                            c = '#';
                        } else {
                            panic!("There can't be # and O in same place!");
                        }
                    }
                }
                print!("{}", c);
            }
            println!(" : sum {}", sum);
        }
    }
}

fn get_closest_square(sq: &Vec<usize>, from: usize) -> Option<&usize> {
    sq.iter().rfind(|s| **s < from)
}

impl InputParser for DishParser {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        self.columns = self.columns.max(line.len());
        for (column, c) in line.chars().enumerate() {
            match c {
                'O' => match self.round_rocks.get_mut(&column) {
                    None => { self.round_rocks.insert(column, vec![self.row]); }
                    Some(v) => { v.push(self.row); }
                }
                '#' => match self.square_rocks.get_mut(&column) {
                    None => { self.square_rocks.insert(column, vec![self.row]); }
                    Some(v) => { v.push(self.row); }
                }

                _ => {}
            }
        };
        self.row += 1;
        Ok(())
    }
}

pub struct DishParser2 {
    pub grid: Vec<Vec<Rock>>,
    // [row][col]
    pub rows: usize,
    pub columns: usize,
}

impl DishParser2 {
    pub fn new() -> Self {
        DishParser2 {
            grid: Vec::new(),
            rows: 0,
            columns: 0,
        }
    }

    pub fn print_stat(&self) {
        println!("Rows {}, cols {}", self.rows, self.columns);
    }
    pub fn tilt_north(&mut self) -> () {
        // process row by row from low to high
        // rocks on row 0  cant move
        for row in 1..self.rows {
            for col in 0..self.columns {
                match self.grid[row][col] {
                    Rock::Round => {
                        let new_row = self.move_north(row, col);
                        self.grid[row][col] = Rock::Empty;
                        self.grid[new_row][col] = Rock::Round;
                    }
                    Rock::Square |
                    Rock::Empty => {}
                }
            }
        }
    }
    pub fn tilt_south(&mut self) -> () {
        // process row by row from low to high
        // rocks on row 0  cant move
        for row in (0..self.rows - 1).rev() {
            for col in 0..self.columns {
                match self.grid[row][col] {
                    Rock::Round => {
                        let new_row = self.move_south(row, col);
                        self.grid[row][col] = Rock::Empty;
                        self.grid[new_row][col] = Rock::Round;
                    }
                    Rock::Square |
                    Rock::Empty => {}
                }
            }
        }
    }

    pub fn tilt_west(&mut self) -> () {
        // process row by row from low to high
        // rocks on row 0  cant move

        for col in 1..self.columns {
            for row in 0..self.rows {
                match self.grid[row][col] {
                    Rock::Round => {
                        let new_col = self.move_west(row, col);
                        self.grid[row][col] = Rock::Empty;
                        self.grid[row][new_col] = Rock::Round;
                    }
                    Rock::Square |
                    Rock::Empty => {}
                }
            }
        }
    }
    pub fn tilt_east(&mut self) {
        // process row by row from low to high
        // rocks on row 0  cant move

        for col in (0..self.columns - 1).rev() {
            for row in 0..self.rows {
                match self.grid[row][col] {
                    Rock::Round => {
                        let new_col = self.move_east(row, col);
                        self.grid[row][col] = Rock::Empty;
                        self.grid[row][new_col] = Rock::Round;
                    }
                    Rock::Square |
                    Rock::Empty => {}
                }
            }
        }
    }

    pub fn calc_weight(&self) -> u64 {
        let mut sum = 0;
        for row in 0..self.rows {
            for col in 0..self.columns {
                match self.grid[row][col] {
                    Rock::Round => {
                        sum += (self.rows - row) as u64; // * 10u64.pow(row as u32);
                    }
                    Rock::Square |
                    Rock::Empty => {}
                }
            }
        }
        println!("{sum}");
        sum
    }
    pub fn spin_cycle(&mut self) -> u64 {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
        self.calc_weight()
    }

    // To find out that this solution works. I printed the calculated load for 200 cycles.
    // analyzed it in Excel to see where it started to get periodic.
    pub fn solve_part2(&mut self) -> u64 {
        let count = 1_000_000_000u64;

        // do a hundred to get to the part where it is cyclic
        let mut load= 0;
        for _ in 0..100 {
            load = self.spin_cycle();
        }
        println!("First 100 !!!");
        let mut cycles = 100usize;
        let mut max_load = load;
        let mut max_cycle = cycles;
        loop {
            let current_load = self.spin_cycle();
            cycles += 1;
            if current_load > max_load {
                max_load = current_load;
                max_cycle = cycles;
            } else if current_load == max_load { // Now we found the next max -> a new period begins
                let period = cycles - max_cycle;
                println!("Period {}, start {} Current cycle {}", period, max_cycle, cycles);
                // Now calculate the load at the end of cycle 1_000_000_000
                let remaining_cycles = (count - cycles as u64) % period as u64; //
                println!(" Remaning {}", remaining_cycles);
                let mut result: u64 = current_load;
                for _ in 0..remaining_cycles {
                    result = self.spin_cycle();
                }
                return result;
            }

        }
    }


    fn move_north(&self, row: usize, col: usize) -> usize {
        let mut start = row;
        while start > 0 && self.grid[start - 1][col] == Rock::Empty {
            start -= 1;
        }
        start
    }

    fn move_south(&self, row: usize, col: usize) -> usize {
        let mut start = row;
        while start < self.rows - 1 && self.grid[start + 1][col] == Rock::Empty {
            start += 1;
        }
        start
    }

    fn move_west(&self, row: usize, col: usize) -> usize {
        let mut start = col;
        while start > 0 && self.grid[row][start - 1] == Rock::Empty {
            start -= 1;
        }
        start
    }

    fn move_east(&self, row: usize, col: usize) -> usize {
        let mut start = col;
        while start < self.columns - 1 && self.grid[row][start + 1] == Rock::Empty {
            start += 1;
        }
        start
    }

    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.columns {
                match self.grid[row][col] {
                    Rock::Round => print!("O"),

                    Rock::Square => print!("#"),
                    Rock::Empty => print!("."),
                }
            }
            println!();
        }
    }
}

impl InputParser for DishParser2 {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.len() == 0 { return Ok(()); }
        self.columns = self.columns.max(line.len());
        let mut this_row: Vec<Rock> = Vec::new();
        for c in line.chars() {
            match c {
                'O' => this_row.push(Rock::Round),

                '#' => this_row.push(Rock::Square),
                _ => this_row.push(Rock::Empty),
            }
        };
        self.grid.push(this_row);
        self.rows += 1;
        Ok(())
    }
}

