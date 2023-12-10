// model types for Day9

use itertools::Itertools;

pub fn part1(input: &Vec<Vec<i64>>) -> i64
{
    input.iter().map(find_next).sum()
}


pub fn find_next(seq:& Vec<i64>) -> i64{
    if seq.iter().all_equal() {
        *seq.last().unwrap()
    } else {
        let diff = seq.iter().tuple_windows().map(|(a,b)|b-a).collect::<Vec<i64>>();
        *seq.last().unwrap() + find_next(&diff)
    }
}

pub fn part2(input: &Vec<Vec<i64>>) -> i64
{
    input.iter().map(find_before).sum()
}
pub fn find_before(seq:& Vec<i64>) -> i64{
    if seq.iter().all_equal() {
        seq[0]
    } else {
        let diff = seq.iter().tuple_windows().map(|(a,b)|b-a).collect::<Vec<i64>>();
        seq[0] - crate::day9::day9_model::find_before(&diff)
    }
}