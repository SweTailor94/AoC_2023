// model types for Day3
#[derive(Debug)]
pub struct PartNumber{
    pub number: u32,
    pub start: usize,
    pub end: usize,
}

impl PartNumber{
    pub fn new(number:u32,start:usize, end:usize) -> Self{
        PartNumber{
            number,
            start,
            end,
        }
    }
}
#[derive(Debug)]
pub struct Symbol {
    pub c: u8,
    pub index: usize
}

pub fn parse_line(line:&str) -> (Vec<PartNumber>,Vec<Symbol>){
    let chars = line.as_bytes();
    let mut part_numbers : Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut found_digit = false;
    let mut number = 0;
    let mut start_index:usize = 0;
    let mut end_index:usize = 0;
    for (i,c) in chars.iter().enumerate(){
         match c {
            b'0'..= b'9' => {
                if found_digit{
                    number = number * 10 + (c-b'0') as u32;
                    end_index = i;
                }else{
                    found_digit = true;
                    number = (c-b'0') as u32;
                    start_index = i;
                    end_index = i;
                }
            }
            other => {
                if found_digit{
                    part_numbers.push(PartNumber::new(number,start_index,end_index));
                    found_digit = false;
                }
                if *other != b'.'{
                    symbols.push(Symbol{
                        c: *other,
                        index: i,
                    });
                }
            }
        };
    };
    if found_digit{
        part_numbers.push(PartNumber::new(number,start_index,end_index));
    };
    (part_numbers,symbols)
}


pub fn part1(input: &Vec<(Vec<PartNumber>,Vec<Symbol>)>) -> u32{
    let mut sum = 0;
    for (i, row) in input.iter().enumerate() {
        for pn in &row.0 {
            let low = if pn.start == 0 {pn.start} else {pn.start -1};
            let high = pn.end+1;
            if  ( row.1.iter().any(|s| s.index >= low && s.index <= high)) ||
                (i > 0 && input[i-1].1.iter().any(|s| s.index >= low && s.index <= high)) ||
                (i < input.len()-1 && input[i+1].1.iter().any(|s| s.index >= low && s.index <= high)) {
                sum+= pn.number;
            }
        }
    };
    sum
}

fn is_symbol_adjacent_to_pn(s: &Symbol, p: &PartNumber) -> Option<u32>{
    let low = if p.start == 0 {p.start} else {p.start -1};
    let high = p.end+1;
    let i = s.index;
    if i >= low  && i <= high{
        Some(p.number)
    } else{
        None
    }

}

pub fn part2(input: &Vec<(Vec<PartNumber>,Vec<Symbol>)>) -> u32{
    let mut sum=0;
    for (i, row) in input.iter().enumerate() {
        for s in &row.1{ // Loop through symbols
            let mut adjacent_parts: Vec<u32> = Vec::new();
            row.0.iter().for_each(|p| if let Some(v) = is_symbol_adjacent_to_pn(s,p){adjacent_parts.push(v)});
            if i > 0 { input[i-1].0.iter().for_each(|p| if let Some(v) = is_symbol_adjacent_to_pn(s,p){adjacent_parts.push(v)}); };
            if i < input.len()-1 { input[i+1].0.iter().for_each(|p| if let Some(v) = is_symbol_adjacent_to_pn(s,p){adjacent_parts.push(v)}); };
            if adjacent_parts.len() == 2{
                sum += adjacent_parts[0] * adjacent_parts[1];
            }
        }
    }
    sum
}

#[cfg(test)]
mod test{
    use crate::day3::day3_model::parse_line;

    #[test]
    fn parse_line_test(){
        let input = ".......*..*.......314............308.......*....*..............156.759.....*................*.......408*954.84..55.......................515";
        let (p,s) = parse_line(input);
        println!("{:?}",p);
        println!("{:?}",s);
        assert_eq!(9, p.len());
        assert_eq!(7,s.len());
    }
}
