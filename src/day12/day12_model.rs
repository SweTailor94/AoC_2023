// model types for Day12

use itertools::{ Itertools};

pub fn valid_combinations(s:&String, groups_target:&Vec<u32>) -> u32 {
    let mut start : Vec<u32> = Vec::new();
    let mut unknown :  Vec<usize> = Vec::new();
    for (i,c) in s.chars().enumerate(){
        match c {
            '?' => {
                unknown.push(i);
                start.push(2);
            }
            '#' => start.push(0),
            '.' => start.push(1),
            _ => panic!("Wrong input"),
        };
    };
    let mut gen = PatternGenerator::new(unknown.len() as u32);
    let mut combinations = 0;
    while let Some(pattern) = gen.next(){
        let test = substitute(&start, &pattern, &unknown);
        let groups = groups(&test);
        combinations += is_valid(&groups,&groups_target)
    }
    combinations
}
fn is_valid(groups: &Vec<u32>, targets:&Vec<u32>) -> u32{
    if groups.eq(targets){
        1
    }else {
        0
    }
}
fn substitute(start: &Vec<u32>, pattern: &Vec<u32>, index:&Vec<usize>) -> Vec<u32>{
    if pattern.len() != index.len(){panic!("pattern length does not match unknown length")}

    let mut res = start.clone();
    for i in 0.. pattern.len(){
        res[index[i]] = pattern[i];
    }
    res
}
fn groups(v: & Vec<u32>) -> Vec<u32>{
    v.iter()
        .group_by(|x|**x == 0)
        .into_iter()
        .filter_map(|(ge0, group)| if ge0 {Some(group.count() as u32)} else {None} ).collect()
}

struct PatternGenerator {
    current:u32,
    size: u32,
    max: u32,
}

impl  PatternGenerator {
    fn new(num_chars:u32) -> Self{
        let base = 2u32;
        let max = base.pow(num_chars);
        PatternGenerator {
            current:0,
            size: num_chars,
            max,
        }
    }

    fn next(&mut self) -> Option<Vec<u32>>{
        if self.current == self.max {
            None
        } else {
            let pattern = (0..self.size).rev().map(|n| (self.current >> n) & 1).collect();
            self.current += 1;
            Some(pattern)
        }
    }
}
