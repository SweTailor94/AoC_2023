// model types for Day2


use std::cmp::max;
#[derive(Debug)]
pub struct GameHand{
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

impl GameHand{
    pub fn new(line: &str) -> Self{
        let mut hand = GameHand{
            red: 0,
            blue: 0,
            green: 0,
        };
        let colours = line.split(',').map(|x|x.trim());
        for c in colours{
            let pair: Vec<&str> = c.split(' ').collect();
            // println!("{:?}",pair);
            let x = pair[1].trim();
            match x {
                "red" => hand.red = pair[0].parse().unwrap(),
                "green" => hand.green = pair[0].parse().unwrap(),
                "blue" => hand.blue = pair[0].parse().unwrap(),
                _ => panic!("Line format failure, unknown colour {}",x),
            }
        }
        hand
    }
}
#[derive(Debug)]
pub struct Game{
    pub id : i32,
    pub hands: Vec<GameHand>,
}

impl Game{
    pub fn get_max_counts(&self) -> (i32,i32,i32){
        let mut red:i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;
        for hand in self.hands.iter(){
            red = max(red, hand.red);
            green = max(green, hand.green);
            blue = max(blue, hand.blue);
        }
        (red,green,blue)
    }
    pub fn get_power(&self) -> i64{
        let (red,green,blue) = self.get_max_counts();
        red as i64 * green as i64 * blue as i64
    }
}

pub fn parse_game(line: &str) -> Game{

    let colon = match  line.find(':'){
        Some(colon) =>  colon,
        None => panic!("Line format failure ,misses colon"),
    };
    let mut game = Game{
        hands: Vec::new(),
        id: line[5..colon].parse().unwrap(),
    };

    let hands: Vec<&str> = line[colon+1 ..].split(';').collect();
    for hand in hands.iter(){
        game.hands.push(GameHand::new(hand));
    }
    game
}
/// Returns Some(game ID) if game is impossible. This wasn't the task apparently :-)
pub fn part1_is_game_impossible(g : &Game) -> Option<i32>{
    const RED_CUBES: i32 = 12;
    const GREEN_CUBES: i32 = 13;
    const BLUE_CUBES: i32 = 14;

    let (red, green, blue) = g.get_max_counts();

    if red > RED_CUBES || green > GREEN_CUBES || blue > BLUE_CUBES {
        Some(g.id)
    } else {
        None
    }
}
pub fn part1_is_game_possible(g : &Game) -> Option<i32>{
    const RED_CUBES: i32 = 12;
    const GREEN_CUBES: i32 = 13;
    const BLUE_CUBES: i32 = 14;

    let (red, green, blue) = g.get_max_counts();

    if red > RED_CUBES || green > GREEN_CUBES || blue > BLUE_CUBES {
        None
    } else {
        Some(g.id)
    }
}
pub fn calc_part1(games: &Vec<Game>) -> i32{
    games.iter().filter_map(part1_is_game_possible).sum()
}

pub fn calc_part2(games: &Vec<Game>) -> i64{
    games.iter().map(|g|g.get_power()).sum()
}