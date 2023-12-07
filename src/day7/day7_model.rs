// model types for Day7



// Cards are numbered
// A -> 1,
// K -2, ... Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.

use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Hand{
    cards: Vec<u8>,
    rank: u8,
    bid: u128,

}

impl Hand {
    pub fn new(line:&str) -> Self{
        let parts = line.split(' ').collect::<Vec<&str>>();
        let bid = parts[1].parse().unwrap();
        let mut cards = vec![];
        for c in parts[0].chars() {
            match c {
                'A' => {cards.push(1);}
                'K' => {cards.push(2);}
                'Q' => {cards.push(3);}
                'J' => {cards.push(4);}
                'T' => {cards.push(5);}
                '9' => {cards.push(6);}
                '8' => {cards.push(7);}
                '7' => {cards.push(8);}
                '6' => {cards.push(9);}
                '5' => {cards.push(10);}
                '4' => {cards.push(11);}
                '3' => {cards.push(12);}
                '2' => {cards.push(13);}
                _ => {panic!("Not a card");}
            }
        }
        let rank = calc_rank(&cards);
        Hand{
            cards,
            rank,
            bid
        }
    }
}


impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank == other.rank {
            for i in 0..5{
                if self.cards[i] != other.cards[i] {return self.cards[i].partial_cmp(&other.cards[i]);}
            }
            return Some(Ordering::Equal);
        } else {
            return self.rank.partial_cmp(&other.rank);
        }
    }
}

impl Ord for Hand{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
#[derive(Debug)]
pub struct Hand2{
    cards: Vec<u8>,
    rank: u8,
    bid: u128,

}
// for part 2 J shall have the largest number
impl crate::day7::day7_model::Hand2 {
    pub fn new(line:&str) -> Self{
        let parts = line.split(' ').collect::<Vec<&str>>();
        let bid = parts[1].parse().unwrap();
        let mut cards = vec![];
        for c in parts[0].chars() {
            match c {
                'A' => {cards.push(1);}
                'K' => {cards.push(2);}
                'Q' => {cards.push(3);}
                'T' => {cards.push(4);}
                '9' => {cards.push(5);}
                '8' => {cards.push(6);}
                '7' => {cards.push(7);}
                '6' => {cards.push(8);}
                '5' => {cards.push(9);}
                '4' => {cards.push(10);}
                '3' => {cards.push(11);}
                '2' => {cards.push(12);}
                'J' => {cards.push(13);} // Joker
                _ => {panic!("Not a card");}
            }
        }
        let rank = calc_rank2(&cards);
        crate::day7::day7_model::Hand2 {
            cards,
            rank,
            bid
        }
    }
}


impl Eq for crate::day7::day7_model::Hand2 {}

impl PartialEq<Self> for crate::day7::day7_model::Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd<Self> for crate::day7::day7_model::Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank == other.rank {
            for i in 0..5{
                if self.cards[i] != other.cards[i] {return self.cards[i].partial_cmp(&other.cards[i]);}
            }
            return Some(Ordering::Equal);
        } else {
            return self.rank.partial_cmp(&other.rank);
        }
    }
}

impl Ord for crate::day7::day7_model::Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

//1 Five of a kind, where all five cards have the same label: AAAAA
//2 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
//3 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
//4 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
//5 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
//6 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
//7 High card, where all cards' labels are distinct: 23456
fn calc_rank(cards:&Vec<u8>) -> u8{
    let mut map :HashMap<u8,u8> =  HashMap::new();
    for i in cards{
        match map.get_mut(i){
            Some(card) => {*card += 1;}
            None => {map.insert(*i,1);}
        }
    }
    match map.len(){
        1 => 1, // Five of a kind
        2 => if map.iter().any(|(_k,v)| *v == 4 ) {2} else {3}, // Four of a kind or Full house
        3 => if map.iter().any(|(_k,v)| *v == 3) {4} else {5}, // Three of a kind or two pais
        4 => 6, // One pair
        5 => 7, // Nothing
        _ => panic!("Impossible"),
    }
}

fn calc_rank2(cards: &Vec<u8>) -> u8{
    let values = cards.iter().unique().sorted().collect::<Vec<&u8>>();
    if values.last().unwrap() == &&13u8 { // Have jokers. Try replacing all jokers with the other card values each at a time.
        let mut possible_hands:Vec<u8> = Vec::new();
        possible_hands.push( calc_rank(&cards.clone()));
        for c in &values[0..values.len()-1] {
            possible_hands.push( calc_rank(&cards.clone().iter().map(|x| if *x==13 {**c}else{*x}).collect() ) );
        }
        *possible_hands.iter().min().unwrap() // Lowest rank is best!!
    } else{
        calc_rank(cards)
    }
}

pub fn calc_part_1(input: &Vec<Hand>) -> u128{
    let rank = input.len() as u128;
    let mut total = 0u128;
    for (i,card) in input.iter().sorted().enumerate(){
        if (rank-i as u128) < 1 {panic!("Something wrong!");}
        total += card.bid * (rank-i as u128);
    }
    total
}
pub fn calc_part_2(input: &Vec<Hand2>) -> u128{
    let rank = input.len() as u128;
    let mut total = 0u128;
    for (i,card) in input.iter().sorted().enumerate(){
        if (rank-i as u128) < 1 {panic!("Something wrong!");}
        total += card.bid * (rank-i as u128);
    }
    total
}