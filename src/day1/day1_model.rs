// model types for Day
pub fn parse_calibration(line: &str) -> u32{
    let ascii : Vec<&u8> = line.as_bytes().iter().filter(|c| **c >= 0x30u8 && **c<= 0x39u8).collect();
    let val = (ascii[0]-0x30) as u32 * 10 +  (ascii[ascii.len()-1]-0x30) as u32;
    val
}

pub fn parse_calibration_2(line : &str) -> u32{

    let mut ental = 0u32;
    let mut tiotal = 0u32;
    let mut maxi = 0usize;
    let mut mini = line.len()-1;
    let text = vec!["nil","one","two","three", "four", "five", "six", "seven", "eight", "nine"];

    let mut check_digit = |min:usize,max:usize,dig:u32| {
        if min <= mini{
            mini = min;
            tiotal = 10*dig;
            println!("tiotal {}",tiotal);
        }
        if max >= maxi {
            maxi = max;
            ental = dig;
            println!("ental {}",ental);
        }
    };

    for i in 1u32 .. 10u32 {
        if let Some((min,max)) = low_high_inecie(line,&format!("{}",i)){
            println!("found {}, ({},{})",i,min,max);
            check_digit(min,max,i) ;
        }
        if let Some((min,max)) = low_high_inecie(line,text[i as usize]){
            println!("found {}, ({},{})",text[i as usize],min,max);
            check_digit(min,max,i);
        }

    }
    if ental == 0 || tiotal == 0 {
        panic!("{}{}",tiotal,ental);
    }
    ental+tiotal
}

fn low_high_inecie(line:&str, pat: &str) -> Option<(usize,usize)>{
    let matches: Vec<_> = line.match_indices(pat).collect();
    if matches.len() > 0 {
        let min = matches.iter().map(|(i, _s)| *i).min().unwrap();
        let max = matches.iter().map(|(i, _s)| *i).max().unwrap();
        Some((min,max))
    }else {
        None
    }
}

#[cfg(test)]
mod test{
    use crate::day1::day1_model::parse_calibration_2;

    #[test]
    fn t1(){
        let inp= "9sixonefour";
        println!("{}",parse_calibration_2(inp));
    }
}