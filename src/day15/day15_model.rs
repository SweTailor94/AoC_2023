use std::hash::{Hash, Hasher};

// model types for Day15
//Determine the ASCII code for the current character of the string.
//Increase the current value by the ASCII code you just determined.
//Set the current value to itself multiplied by 17.
//Set the current value to the remainder of dividing itself by 256.
pub fn hash(s: &str)->u8{
    let mut cur = 0;
    for i in s.as_bytes(){
        cur += *i as i32;
        cur *= 17;
        cur %= 256;
    }
    cur as u8
}

#[derive(Clone)]
pub struct Lens{
    label:String,
    focal_point: u8,
}
impl Lens {
    fn _new(l: String, fp:u8) -> Self{
        Lens{
            label: l,
            focal_point: fp,
        }
    }
}

impl Hash for Lens{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state)
    }
}
impl PartialEq<Self> for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

#[derive(Clone)]
pub struct LensBox{
    pub slots: Vec<Lens>,
}

impl Default for LensBox {
    fn default() -> Self {
        LensBox {
            slots: Vec::new(),
        }
    }
}
impl LensBox{
    pub fn add_or_replace(&mut self, label: String, focal_point:u8){
        for i in 0..self.slots.len(){
            if self.slots[i].label == label {
                self.slots[i].focal_point = focal_point;
                return;
            }
        }
        self.slots.push(Lens{
            label,
            focal_point,
        });
    }

    pub fn remove(&mut self, label:String){
        self.slots.retain(|l| l.label != label);
    }

}

pub struct LensArray{
    pub boxes:Vec<LensBox>,
}

impl LensArray{
    pub fn add_rule(&mut self, r: &str) {
       let ascii =  r.as_bytes();
        let mut i= 0;
        while ascii[i] != b'=' && ascii[i] != b'-'{
            i += 1;
        }
        let label = r[0..i].to_string();
        let box_nr = hash(&label) as usize;

        match ascii[i] {
            b'=' => {
                // Add or change lens
                let fp = r[(i+1)..].parse::<u8>().unwrap();
                self.boxes[box_nr].add_or_replace(label, fp);
            }
            b'-' => {
                // remove lens
                self.boxes[box_nr].remove(label);
            }
            _ => {
                panic!("Should not be possible")
            }
        }

        ()
    }
    pub fn calc_val_part2(&self) -> u64{
        let mut sum = 0u64;
        for (i,b) in self.boxes.iter().enumerate(){
            if b.slots.len() > 0{
                for (j,l) in b.slots.iter().enumerate() {
                    sum += (i+1) as u64 * (j+1) as u64 * l.focal_point as u64;
                }
            }
        }
        sum
    }
}

impl LensArray{
    pub fn new() -> Self{
        LensArray{
            boxes: vec![LensBox::default();256],
        }
    }
}