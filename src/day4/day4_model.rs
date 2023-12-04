// model types for Day4


use std::collections::HashMap;

pub fn parse_card(line:&str) -> (Vec<u32>, Vec<u32>){
    let first:Vec<&str> = line.split(':').collect();
    let second: Vec<&str> = first[1].split('|').collect();
    let winning_numbers: Vec<u32> = second[0].trim().split(' ').filter_map(|n| match n.parse::<u32>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }).collect();
    let my_numbers:  Vec<u32> = second[1].trim().split(' ').filter_map(|n| match n.parse::<u32>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }).collect();
    (winning_numbers,my_numbers)
}

fn get_points(winn: &Vec<u32>, my: &Vec<u32> ) -> u32{
    let hits = my.iter().filter_map(|m|if winn.contains(m){Some(m)}else{None}).count() as u32;
    match hits {
        0 => 0,
        1 => 1,
        v=> 1 << (v-1),
    }
}

fn get_wins(winn: &Vec<u32>, my: &Vec<u32> ) -> u32{
    my.iter().filter_map(|m|if winn.contains(m){Some(m)}else{None}).count() as u32
}

pub fn calc_points_part1( input: &Vec<(Vec<u32>,Vec<u32>)>) -> u32{
    let mut sum = 0;
    for (win, my) in input{
        sum = sum +get_points(win, my) ;
    };
    sum
}

pub fn calc_cards_part2(input: &Vec<(Vec<u32>,Vec<u32>)>) -> u32{
    let cards_w_wins :Vec<(usize,u32)> = input.iter().enumerate().map(|(i,(w, m))|(i,get_wins(w,m))).collect();
    let mut number_of_cards = 0;
    let mut more_cards: HashMap<usize,u32> = HashMap::new();
    for (i,wins_on_card) in cards_w_wins{
        let extra_cards = match more_cards.get(&i){
            Some(count) => *count,
            None => 0,
        };
        number_of_cards += 1 + extra_cards ;
        if wins_on_card > 0{
            for n in i+1 ..= i+wins_on_card as usize  {
                match more_cards.get_mut(&n){
                    Some(o) => {*o += 1+extra_cards;}
                    None => {more_cards.insert(n,1+extra_cards);}
                };
            }
        }
    }
    number_of_cards
}

#[cfg(test)]
mod test{
    use crate::day4::day4_model::get_points;
    #[test]
    fn test_points() {
        let winn = vec![1,2,3,4,5];
        let my = vec![10,3,9,5,12,15,17];
        let p = get_points(winn, my);
        assert_eq!(2,p);
    }
}