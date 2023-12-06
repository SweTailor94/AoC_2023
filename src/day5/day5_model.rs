// model types for Day5

use std::ops::Range;
use crate::input::InputParser;
use rayon::prelude::*;

pub struct SeedMap {
    pub seeds: Vec<i64>,
    pub seed_to_soil: Translator,
    pub soil_to_fert: Translator,
    pub fert_to_water: Translator,
    pub water_to_light: Translator,
    pub light_to_temp: Translator,
    pub temp_to_hum: Translator,
    pub hum_to_location: Translator,
    state: ParseState,
}

impl SeedMap {
    pub fn new() -> Self {
        SeedMap {
            seeds: Vec::new(),
            seed_to_soil: Translator::new(),
            soil_to_fert: Translator::new(),
            fert_to_water: Translator::new(),
            water_to_light: Translator::new(),
            light_to_temp: Translator::new(),
            temp_to_hum: Translator::new(),
            hum_to_location: Translator::new(),
            state: ParseState::Seeds,
        }
    }

    pub fn calc_part_1(&self) -> i64{
        self.seeds
            .iter()
            .map(|s| self.hum_to_location
                .to(self.temp_to_hum
                    .to(self.light_to_temp
                        .to(self.water_to_light
                            .to(self.fert_to_water
                                .to(self.soil_to_fert
                                    .to(self.seed_to_soil
                                        .to(*s))))))))
            .min().unwrap()
    }

    // The part2 solution is a brute force that took a little more than an hour on my computer.
    // Since it terminated while I was thinking of a smarter algorithm it did terminate so I had solved it.
    // If I have time in the future I may update this solution.
    pub fn calc_part_2(&self) -> i64{
        //let mut temp_mins= Vec::new();
        (0..self.seeds.len()).step_by(2).collect::<Vec<_>>()
            .into_par_iter().map(|i|{
            let start = self.seeds[i];
            (start..(start+self.seeds[i+1]))
                .map(|s| self.hum_to_location
                    .to(self.temp_to_hum
                        .to(self.light_to_temp
                            .to(self.water_to_light
                                .to(self.fert_to_water
                                    .to(self.soil_to_fert
                                        .to(self.seed_to_soil
                                            .to(s))))))))
                .min().unwrap()
            }).min().unwrap()
    }
}

impl InputParser for SeedMap {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        match self.state {
            ParseState::Seeds => {
                if line.trim().is_empty() {
                    self.state = ParseState::SeedToSoil;
                } else {
                    self.seeds
                        = line.split(':')
                        .skip(1)
                        .next()
                        .unwrap()
                        .trim()
                        .split(' ')
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<i64>>();
                }
            }
            ParseState::SeedToSoil => {
                if line.trim().is_empty() {
                    self.state = ParseState::SoilToFertilizer;
                } else if line.starts_with(state_to_text(&self.state)) {
                } else {
                    self.seed_to_soil.add_range(RangeTranslation::new(line));
                }
            }
            ParseState::SoilToFertilizer => {
                if line.trim().is_empty() {
                    self.state = ParseState::FertilizerToWater;
                } else if line.starts_with(state_to_text(&self.state)) {
                } else {
                    self.soil_to_fert.add_range(RangeTranslation::new(line));
                }
            }
            ParseState::FertilizerToWater => {
                if line.trim().is_empty() {
                    self.state = ParseState::WaterToLight;
                } else if line.starts_with(state_to_text(&self.state)) {
                } else {
                    self.fert_to_water.add_range(RangeTranslation::new(line));
                }
            }
            ParseState::WaterToLight => {
                if line.trim().is_empty() {
                    self.state = ParseState::LightToTemperature;
                } else if line.starts_with(state_to_text(&self.state)) {
                } else {
                    self.water_to_light.add_range(RangeTranslation::new(line));
                }
            }
            ParseState::LightToTemperature => {
                if line.trim().is_empty() {
                    self.state = ParseState::TemperatureToHumidity;
                } else if line.starts_with(state_to_text(&self.state)) {
                } else {
                    self.light_to_temp.add_range(RangeTranslation::new(line));
                }
            }
            ParseState::TemperatureToHumidity => {
                if line.trim().is_empty() {
                    self.state = ParseState::HumidityToLocation;
                } else if line.starts_with(state_to_text(&self.state)) {
                } else {
                    self.temp_to_hum.add_range(RangeTranslation::new(line));
                }
            }
            ParseState::HumidityToLocation => {
                if line.trim().is_empty() {
                    self.state = ParseState::Done;
                } else if line.starts_with(state_to_text(&self.state)) {
                } else {
                    self.hum_to_location.add_range(RangeTranslation::new(line));
                }
            }
            ParseState::Done => {}
        }
        Ok(())
    }
}

enum ParseState {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    Done,
}

fn state_to_text(s: &ParseState) -> &str {
    match s {
        ParseState::Seeds => "seeds",
        ParseState::SeedToSoil => "seed-to-soil",
        ParseState::SoilToFertilizer => "soil-to-fertilizer",
        ParseState::FertilizerToWater => "fertilizer-to-water",
        ParseState::WaterToLight => "water-to-light",
        ParseState::LightToTemperature => "light-to-temperature",
        ParseState::TemperatureToHumidity => "temperature-to-humidity",
        ParseState::HumidityToLocation => "humidity-to-location",
        ParseState::Done => "Done",
    }
}


pub struct RangeTranslation {
    range: Range<i64>,
    offset: i64,
}

impl RangeTranslation {
    pub fn new(input: &str) -> Self {
        let parts = input.split(' ')
            .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

        RangeTranslation {
            range: (parts[1]..parts[1] + parts[2]),
            offset: parts[0] - parts[1],
        }
    }
    pub fn translate(&self, i: i64) -> Option<i64> {
        if self.range.contains(&i) {
            Some(i + self.offset)
        } else {
            None
        }
    }
}

pub struct Translator {
    ranges: Vec<RangeTranslation>,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            ranges: Vec::new(),
        }
    }

    pub fn add_range(&mut self, range: RangeTranslation) {
        self.ranges.push(range);
    }

    pub fn to(&self, i: i64) -> i64 {
        for r in &self.ranges {
            if let Some(o) = r.translate(i) {
                return o;
            }
        };
        return i;
    }
}