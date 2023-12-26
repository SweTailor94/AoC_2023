// model types for Day13

use crate::input::InputParser;


#[derive(Debug,Clone)]
enum Mirror {
    NotFoundYet,
    Line(usize), // * 100
    Col(usize), //
}
#[derive(Debug,Clone)]
pub struct MirrorPattern{
    lines: Vec<u64>,
    cols: Vec<u64>,
    mirror:Mirror,
}
impl MirrorPattern{
    pub fn new() -> Self{
        MirrorPattern{
            lines: vec![],
            cols: vec![],
            mirror: Mirror::NotFoundYet,
        }
    }


    pub fn find_mirror(&mut self){
        for i in 0 .. self.lines.len() -1 {
            if self.lines[i] == self.lines[i+1] &&
                is_mirror(&self.lines, i){

                self.mirror = Mirror::Line(i);
                return;
            }
        }
        for i in 0 .. self.cols.len() -1 {
            if self.cols[i] == self.cols[i+1] &&
                is_mirror(&self.cols, i){
                self.mirror = Mirror::Col(i);
                return;
            }
        }
    }
    pub fn find_mirror_if_cleaned(&mut self){
        for i in 0 .. self.lines.len() -1 {
            if is_mirror_cleaned(&self.lines, i){

                self.mirror = Mirror::Line(i);
                return;
            }
        }
        for i in 0 .. self.cols.len() -1 {
            if  is_mirror_cleaned(&self.cols, i){
                self.mirror = Mirror::Col(i);
                return;
            }
        }
    }
    pub fn calc_pattern_value(&self) -> u64{
        match self.mirror {
            Mirror::NotFoundYet => {panic!("Cant get value before finding mirror")}
            Mirror::Line(v) => {(v as u64 +1) *100}
            Mirror::Col(v) => { v as u64+1 }
        }
    }
}
fn is_mirror(v:&Vec<u64>, i:usize)->bool{
    let mut left = i;
    let mut right = i+1;
    loop{
        if v[left] != v[right] {
            return false;
        }
        right += 1;
        if left == 0 || right == v.len() {
            break;
        }
        left -= 1;
    }
    return true;
}
fn is_mirror_cleaned(v:&Vec<u64>, i:usize)->bool{
    let mut left = i;
    let mut right = i+1;
    let mut tried_clean = false;
    loop{
        if v[left] != v[right] {
            if tried_clean || !can_be_cleaned(v[left],v[right]) {
                return false;
            }else{
                tried_clean = true;
            }
        }
        right += 1;
        if left == 0 || right == v.len() {
            break;
        }
        left -= 1;
    }
    return tried_clean; // Must have changed one to not use old mirror Line.
}

fn can_be_cleaned(left:u64,right:u64) -> bool{
    let mut x = left^right;
    while x > 0 {
        if x % 2 == 1 {
            if x == 1 {
                return true;
            } else{
                return false;
            }
        }
        x = x >> 1;
    }
    return false;
}

#[derive(Debug)]
pub struct PatternParser{
    mirror_patterns: Vec<MirrorPattern>,
    work_pattern:MirrorPattern,
    work_columns: Vec<u64>,
}

impl PatternParser{
    pub fn new() -> Self{
        PatternParser{
            mirror_patterns: vec![],
            work_pattern: MirrorPattern::new(),
            work_columns: vec![],
        }
    }

    pub fn solve_p1(&mut self) -> u64{
        let mut sum = 0u64;
        for m in self.mirror_patterns.iter_mut(){
            m.find_mirror();
            sum += m.calc_pattern_value();
        }
        sum
    }

    pub fn solve_p2(&mut self) -> u64{
        let mut sum = 0u64;
        for m in self.mirror_patterns.iter_mut(){
            m.find_mirror_if_cleaned();
            sum += m.calc_pattern_value();
        }
        sum

    }
}

impl InputParser for PatternParser{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {
            self.work_pattern.cols = self.work_columns.clone();
            self.mirror_patterns.push(self.work_pattern.clone());
            self.work_pattern = MirrorPattern::new();
            self.work_columns.clear();
        } else {
            if self.work_columns.len() == 0 {
                self.work_columns = vec![0u64;line.len()];
            }
            let mut line_val = 0u64;
            for (i, c) in line.chars().enumerate(){
                let adder = match c {
                    '#' => 1u64,
                    '.' => 0u64,
                    _ => panic!("There should be only # or . on a non empty line.")
                };
                self.work_columns[i] = (self.work_columns[i] << 1) + adder;
                line_val = (line_val << 1) + adder;
            }
            self.work_pattern.lines.push(line_val);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test{
    use crate::day13::day13_model::can_be_cleaned;

    #[test]
    fn test_can_be_cleaned(){
        assert_eq!(can_be_cleaned(0b00100000, 0b00100100), true);
        assert_eq!(can_be_cleaned(0b00100000, 0b10100000), true);
        assert_eq!(can_be_cleaned(0b00100000, 0b10100100), false);
        assert_eq!(can_be_cleaned(0b00100001, 0b00100100), false);
        assert_eq!(can_be_cleaned(0b00100010, 0b00100110), true);
    }

}