use crate::utils;
use std::{iter::Iterator, collections::VecDeque};

#[derive(Debug)]
struct RangeRec {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Debug)]
struct RangeMap(Vec<RangeRec>);

impl RangeMap {
    pub fn map(&self, v: u64) -> u64 {
        for i in self.0.iter() {
            if v >= i.src_start && v < i.src_start + i.len {
                return v - i.src_start + i.dst_start;
            }
        }
        v
    }
}

#[derive(Debug)]
struct InputData {
    seeds: Vec<u64>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl InputData {
    pub fn load() -> Self {
        let mut lines = utils::read_lines("data/day5.txt").collect::<VecDeque<_>>();

        let seeds = lines.pop_front().unwrap().split(' ').skip(1).map(|s| s.parse::<u64>().unwrap()).collect();

        let seed_to_soil = Self::read_range_vec(&mut lines, "seed-to-soil map:");
        let soil_to_fertilizer = Self::read_range_vec(&mut lines, "soil-to-fertilizer map:");
        let fertilizer_to_water = Self::read_range_vec(&mut lines, "fertilizer-to-water map:");
        let water_to_light = Self::read_range_vec(&mut lines, "water-to-light map:");
        let light_to_temperature = Self::read_range_vec(&mut lines, "light-to-temperature map:");
        let temperature_to_humidity = Self::read_range_vec(&mut lines, "temperature-to-humidity map:");
        let humidity_to_location = Self::read_range_vec(&mut lines, "humidity-to-location map:");

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn read_range_vec(lines: &mut VecDeque<String>, expected_title: &'static str) -> RangeMap {
        let mut records = Vec::new();
        let l = lines.pop_front().unwrap();
        assert_eq!(l, expected_title);

        while let Some(l) = lines.front() {
            if l.chars().next().unwrap().is_alphabetic() {
                break;
            }
            let l = lines.pop_front().unwrap();
            let tokens: Vec<u64> = l.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            assert_eq!(tokens.len(), 3);
            records.push(RangeRec {
                dst_start: tokens[0],
                src_start: tokens[1],
                len: tokens[2],
            });
        }

        RangeMap(records)
    }
}

pub fn test1() {
    let data = InputData::load();
    let mut min = None;
    for seed in data.seeds.iter() {
        let mapped = *seed;
        let mapped = data.seed_to_soil.map(mapped);
        let mapped = data.soil_to_fertilizer.map(mapped);
        let mapped = data.fertilizer_to_water.map(mapped);
        let mapped = data.water_to_light.map(mapped);
        let mapped = data.light_to_temperature.map(mapped);
        let mapped = data.temperature_to_humidity.map(mapped);
        let mapped = data.humidity_to_location.map(mapped);

        min = match min {
            None => Some(mapped),
            Some(v) if v >= mapped => Some(mapped),
            Some(v) => Some(v),
        }
    }
    println!("{min:?}");
}

pub fn test2() {
    // It was supposed to be too slow to brute force
    // ...
    // I didn't care.
    // <img src='di_caprio_cheers_meme.gif'></img>
    let data = InputData::load();
    let mut min = None;
    for i in (0..data.seeds.len()).step_by(2) {
        println!("{} to {}..", data.seeds[i], data.seeds[i]+data.seeds[i+1]);
        for seed in data.seeds[i]..data.seeds[i]+data.seeds[i+1] {
            let mapped = seed;
            let mapped = data.seed_to_soil.map(mapped);
            let mapped = data.soil_to_fertilizer.map(mapped);
            let mapped = data.fertilizer_to_water.map(mapped);
            let mapped = data.water_to_light.map(mapped);
            let mapped = data.light_to_temperature.map(mapped);
            let mapped = data.temperature_to_humidity.map(mapped);
            let mapped = data.humidity_to_location.map(mapped);

            min = match min {
                None => Some(mapped),
                Some(v) if v >= mapped => Some(mapped),
                Some(v) => Some(v),
            }
        }
        println!("\tdone!");
    }
    println!("{min:?}");
}
