// model types for Day8

use std::collections::{HashMap, HashSet};
use crate::input::InputParser;
use prime_factorization::Factorization;

#[derive(Debug)]
pub struct DessertMap{
    pub directions: Vec<usize>,
    pub nodes: HashMap<String,[String;2]>,
    pub ends_w_a: Vec<String>,
    pub ends_w_z: Vec<String>,
    is_firstline: bool,
}

impl DessertMap {
    pub fn new() -> Self{
        DessertMap{
            directions: Vec::new(),
            nodes: HashMap::new(),
            ends_w_a: Vec::new(),
            ends_w_z: Vec::new(),
            is_firstline: true,
        }
    }

    pub fn follow_directions(&self,start: String,end: String ) -> u32{
        let mut current = start;
        let mut count= 0;
        let mut index = 0usize;
        while current != end {
            let t = self.nodes.get(&current).unwrap();
            current = t[self.directions[index]].clone();
            count += 1;
            index = (index + 1)  % self.directions.len();
        }
        count
    }

    // First naive try on part b. May have terminated eventually.
    pub fn follow_directions_multi(&self) -> u32{
        let mut current = self.ends_w_a.clone();
        let mut count= 0;
        let mut index = 0usize;
        let number_of_ghosts = current.len();
        println!("Ghosts: {}",number_of_ghosts);
        let mut z_count = 0;
        while z_count != number_of_ghosts {
            if z_count > 0 { println!("{} {:?}",count , current);}
            let mut temp : Vec<String> = Vec::new();
            for node in current{
                let t = self.nodes.get(&node).unwrap();
                temp.push( t[self.directions[index]].clone());
            }
            current = temp;
            count += 1;
            index = (index + 1)  % self.directions.len();
            z_count = current.iter().fold(0, |c,x|if x.ends_with('Z') {c+1}else{c});
        }
        count
    }

    pub fn follow_directions_ends_z(&self,start: &String) -> (u32, String){
        let mut current = start;
        let mut count= 0;
        let mut index = 0usize;
        let mut done = false;
        while !done {
            let t = self.nodes.get(current).unwrap();
            current = &t[self.directions[index]];
            count += 1;
            index = (index + 1)  % self.directions.len();
            done = current.ends_with('Z')
        }
        (count, current.clone())
    }
    pub fn part_2(&self) -> u64{
        let mut cycles: HashSet<u64> = HashSet::new();
        for start in &self.ends_w_a{
            let (_,end_node) = self.follow_directions_ends_z(&start); // This is to find the end node for corresponding start
            let (count1, _) = self.follow_directions_ends_z(&end_node); // This one is the period of the sequence
            Factorization::run(count1 as u64).factors.iter().for_each(|f| {cycles.insert(*f);()});
        }
        // 263*73*43*67*79*61*59
        cycles.iter().product()
    }

}

impl InputParser for DessertMap{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {self.is_firstline = false;
            return Ok(());
        }
        if self.is_firstline {
            self.directions = line.chars().map(
                |x| match x {
                    'L' => 0 as usize,
                    'R' => 1 as usize,
                    _ => panic!("Faulti 1.st line")
                }).collect();
        } else {
            let key_value = line.split('=').collect::<Vec<_>>();
            let key = key_value[0].trim().to_string();
            if key.ends_with('A') {
                self.ends_w_a.push(key.clone());
            }
            if key.ends_with('Z') {
                self.ends_w_z.push(key.clone());
            }
            let p: &[_] = &[' ','(',')'];
            let values = key_value[1]
                .trim_matches(p)
                .split(',')
                .map(|x|x.trim().to_string())
                .collect::<Vec<_>>();
            self.nodes.insert(key, [values[0].clone(),values[1].clone()]);
        };
        Ok(())
    }
}