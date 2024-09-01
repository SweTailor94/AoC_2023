// model types for Day18

pub struct BoolMatrix{
    rows: usize,
    cols: usize,
    values: Vec<Vec<bool>>,
}

impl BoolMatrix {
    pub fn print(&self, true_sign: &str, false_sign: &str) {
        for row in (0..self.rows).rev() {
            print!("{:>3} ",row);
            for col in 0..self.cols{
                if self.values[row][col] {
                    print!("{}",true_sign);
                } else {
                    print!("{}",false_sign);
                }
            }
            println!()
        }
        print!("    ");
        for col in 0..self.cols{
            print!("{}",col%10);
        }
        println!();
    }
}

impl BoolMatrix{
    pub fn zeroes(r: usize, c: usize) -> Self{
        BoolMatrix {
            rows: r,
            cols: c,
            values: vec![vec![false; c]; r],
        }
    }

    pub fn get(&mut self, r: usize, c: usize) -> &mut bool{
        &mut self.values[r][c]
    }

    pub fn set(&mut self, r: usize, c: usize) {
        self.values[r][c] = true;
    }
    pub fn reset(&mut self, r: usize, c: usize) {
        self.values[r][c] = false;
    }

}