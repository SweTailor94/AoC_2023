// model types for Day16

use std::collections::{HashMap, HashSet};
use crate::day16::day16_model::Direction::{East, North, South, West};
use crate::input::InputParser;

pub enum Mirror{
    ///  '/'
    Positive,
    /// '\'
    Negative,
    /// '|'
    SplitVert,
    /// '-'
    SplitHor,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction{
    North,
    East,
    South,
    West,
}

#[derive(Clone,Eq, PartialEq, Hash)]
pub struct Beam{
    pub pos:(i32,i32), // (row,col)
    pub direction:Direction,
}

impl Beam{
    pub fn step(&mut self){
        let (row,col) = self.pos;
        match self.direction{
            Direction::North => {self.pos = (row-1, col);}
            Direction::East => {self.pos = (row, col+1);}
            Direction::South => {self.pos = (row+1,col);}
            Direction::West => {self.pos = (row, col-1);}
        }
    }
}

pub struct Contraption{
    pub objects : HashMap<(i32,i32),Mirror>,
    rows:i32,
    cols:i32,
}

impl InputParser for Contraption{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {return Ok(());}
        self.cols = self.cols.max( line.len() as i32);
        line.chars().enumerate().for_each(|(i,c)|{
            match c{
                '/' => { self.objects.insert((self.rows,i as i32),Mirror::Positive); }
                '\\' => { self.objects.insert((self.rows,i as i32),Mirror::Negative); }
                '|' =>  { self.objects.insert((self.rows,i as i32),Mirror::SplitVert); }
                '-' => { self.objects.insert((self.rows,i as i32),Mirror::SplitHor); }
                _ => {}
            };
        });
        self.rows += 1;
        Ok(())
    }
}

impl Contraption{
    pub fn new() -> Self{
        Contraption{
            objects: Default::default(),
            rows : 0,
            cols : 0,
        }
    }

    pub fn print_stats(&self) {
        println!("Rows {} cols {} #objects {}",self.rows, self.cols, self.objects.len());
    }

    pub fn run_beam_get_energized_tiles(&self) -> usize {
        self.energized_tiles(Beam{
            pos: (0,-1),
            direction: Direction::East,
        })
    }

    pub fn find_maximum(&self) -> usize{
        let mut max_energize = 0usize;
        for r in 0..self.rows{
            max_energize = max_energize.max(self.energized_tiles(Beam{
                pos: (r, -1),
                direction: Direction::East,
            }));
            max_energize = max_energize.max(self.energized_tiles(Beam{
                pos: (r, self.cols),
                direction: Direction::West,
            }));
        }
        for c in 0..self.cols{
            max_energize = max_energize.max(self.energized_tiles(Beam{
                pos: (-1,c),
                direction: Direction::South,
            }));
            max_energize = max_energize.max(self.energized_tiles(Beam{
                pos: (self.rows,c),
                direction: Direction::North,
            }));
        }
        max_energize
    }

    pub fn energized_tiles(&self, start: Beam) -> usize{
        let mut beams :HashSet<Beam> = HashSet::new();
        beams.insert( start);
        let mut energized_tiles: HashSet<(i32,i32)> = HashSet::new();

        let mut equal_count = 0;
        while !beams.is_empty() {
            let beam_clones = beams.clone();
            beams.clear();
            let before = energized_tiles.len();
            for mut b in beam_clones.into_iter(){
                b.step();
                if self.is_inside(b.pos) {
                    energized_tiles.insert(b.pos);
                    match self.objects.get(&b.pos){
                        None => {
                            beams.insert(b);
                        }
                        Some(Mirror::Negative)  => { //  '\'
                            match b.direction{
                                North => {b.direction = West;}
                                East => {b.direction = South;}
                                South => {b.direction = East;}
                                West => {b.direction = North;}
                            }
                            beams.insert(b);
                        }
                        Some(Mirror::Positive) => { // '/'
                            match b.direction{
                                North => {b.direction = East;}
                                East => {b.direction = North;}
                                South => {b.direction = West;}
                                West => {b.direction = South;}
                            }
                            beams.insert(b);
                        }
                        Some(Mirror::SplitVert) => {
                            match b.direction{
                                East |
                                West => {
                                    beams.insert(Beam{
                                        pos: b.pos,
                                        direction: North,
                                    });
                                    beams.insert(Beam{
                                        pos: b.pos,
                                        direction:South,
                                    });
                                }
                                _ => {
                                    beams.insert(b);
                                }
                            }
                        }
                        Some(Mirror::SplitHor) => {
                            match b.direction{
                                North |
                                South => {
                                    beams.insert(Beam{
                                        pos: b.pos,
                                        direction: East,
                                    });
                                    beams.insert(Beam{
                                        pos: b.pos,
                                        direction:West,
                                    });
                                }
                                _ => {
                                    beams.insert(b);
                                }
                            }
                        }
                    }
                }

            }
            if energized_tiles.len() == before{
                equal_count += 1;
                if equal_count == 10{break;}
            } else {
                equal_count = 0;
            }

        }
        energized_tiles.len()
    }

    fn is_inside(&self, pos: (i32, i32)) -> bool {
        let (row, col) = pos;
        row >= 0 && row < self.rows && col >= 0 && col < self.cols
    }
}
