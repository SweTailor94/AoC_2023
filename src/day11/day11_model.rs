// model types for Day11

use crate::input::InputParser;
#[derive(Debug)]
pub struct Space{
    galaxies: Vec<(u64,u64)>,
    lines: u64,
    cols: u64,
}

impl Space{
    pub fn new() -> Self{
        Space{
            galaxies: Vec::new(),
            lines: 0,
            cols: 0,
        }
    }

    pub fn num_galaxies(&self) -> u64{
        self.galaxies.len() as u64
    }

    pub fn calc_total_dist(&mut self, part: i32) ->u128{
        // Expand the universe
        let additional = if part == 1 {
            1
        } else {
            999_999
        };
        self.expand(additional);
        let mut sum:u128 = 0;
        for i in 0..self.galaxies.len()-1{
            for j in i+1..self.galaxies.len() {
                sum = sum + dist(self.galaxies[i],self.galaxies[j]) as u128;
            }
        }

        sum
    }

    fn expand(&mut self, additional:u64) {
        // First do rows
        let mut offset = 0;
        let mut i = 0;
        let mut expanded_rows: Vec<(u64,u64)> = Vec::new();
        for row in 0..self.lines{
            if self.galaxies[i].0 != row{
                offset += additional;
            } else {
                while i < self.galaxies.len() && self.galaxies[i].0 == row {
                    let (x, y) = self.galaxies[i];
                    expanded_rows.push( (x + offset, y) );
                    i += 1;
                }
            }
        }
        self.galaxies.clear();
        // Now columns
        expanded_rows.sort_by(|(_,col),(_,col2)|col.cmp(col2));
        let mut offset = 0;
        let mut i = 0;
        for col in 0..self.cols{
            if expanded_rows[i].1 != col{
                offset += additional;
            } else {
                while i < expanded_rows.len() && expanded_rows[i].1 == col {
                    let (x, y) = expanded_rows[i];
                    self.galaxies.push( (x , offset + y) );
                    i += 1;
                }
            }
        }
    } // Expand()

} //impl

fn dist(p0: (u64, u64), p1: (u64, u64)) -> u64 {
    p1.0.abs_diff(p0.0) + p1.1.abs_diff(p0.1)
}

impl InputParser  for Space {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        self.cols = line.len() as u64;
        for (i, s) in line.chars().enumerate(){
            if s == '#' {
                self.galaxies.push((self.lines, i as u64));
            }
        }
        self.lines += 1;
        Ok(())
    }
}