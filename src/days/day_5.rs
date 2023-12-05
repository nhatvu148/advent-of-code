use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};

pub struct MapData {
    pub seeds: Vec<u128>,
    pub seed_to_soil: Vec<(u128, u128, u128)>,
    pub soil_to_fertilizer: Vec<(u128, u128, u128)>,
    pub fertilizer_to_water: Vec<(u128, u128, u128)>,
    pub water_to_light: Vec<(u128, u128, u128)>,
    pub light_to_temperature: Vec<(u128, u128, u128)>,
    pub temperature_to_humidity: Vec<(u128, u128, u128)>,
    pub humidity_to_location: Vec<(u128, u128, u128)>,
}

pub fn process_file(file_path: &str) -> io::Result<MapData> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sections = vec![];
    let mut current_section = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            // If the line is empty, start a new section
            sections.push(std::mem::take(&mut current_section));
        } else {
            current_section.push(line);
        }
    }

    // Process the last section
    sections.push(current_section);

    let mut seeds = Vec::new();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    for section in sections {
        let mut lines = section.into_iter().filter(|s| !s.trim().is_empty());

        if let Some(header) = lines.next() {
            let mut parts = header.split(':').map(|s| s.trim());

            match parts.next() {
                Some("seeds") => seeds.extend(
                    parts
                        .next()
                        .unwrap_or_default()
                        .split_whitespace()
                        .filter_map(|s| s.parse::<u128>().ok()),
                ),
                Some("seed-to-soil map") => seed_to_soil.extend(parse_tuple_lines(&mut lines)),
                Some("soil-to-fertilizer map") => {
                    soil_to_fertilizer.extend(parse_tuple_lines(&mut lines))
                }
                Some("fertilizer-to-water map") => {
                    fertilizer_to_water.extend(parse_tuple_lines(&mut lines))
                }
                Some("water-to-light map") => water_to_light.extend(parse_tuple_lines(&mut lines)),
                Some("light-to-temperature map") => {
                    light_to_temperature.extend(parse_tuple_lines(&mut lines))
                }
                Some("temperature-to-humidity map") => {
                    temperature_to_humidity.extend(parse_tuple_lines(&mut lines))
                }
                Some("humidity-to-location map") => {
                    humidity_to_location.extend(parse_tuple_lines(&mut lines))
                }
                _ => {}
            }
        }
    }

    Ok(MapData {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    })
}

pub fn find_lowest_location(map_data: &MapData) -> (u128, u128) {
    let MapData {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    } = map_data;
    let mut current_location;

    let transform_chain = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    let mut min_location = u128::MAX;

    for &seed in seeds.iter() {
        current_location = seed;

        for transform_rule in transform_chain.iter() {
            let mut temp = 0;
            let mut is_within_range = false;
            for &(dest_start, src_start, range_len) in transform_rule.iter() {
                if src_start <= current_location && current_location < src_start + range_len {
                    is_within_range = true;
                    temp = current_location - src_start + dest_start;
                    break;
                }
            }
            if is_within_range {
                current_location = temp;
            }
        }

        if current_location < min_location {
            min_location = current_location;
        }
    }

    let min_locations: Vec<u128> = seeds
        .par_chunks(2)
        .map(|chunk| {
            if let [seed_start, seed_range] = chunk {
                let seed_start = *seed_start;
                let seed_range = *seed_range;

                (0..seed_range)
                    .map(|i| {
                        let mut current_location = seed_start + i;

                        for transform_rule in transform_chain.iter() {
                            let mut temp = 0;
                            let mut is_within_range = false;
                            for &(dest_start, src_start, range_len) in transform_rule.iter() {
                                if src_start <= current_location
                                    && current_location < src_start + range_len
                                {
                                    is_within_range = true;
                                    temp = current_location - src_start + dest_start;
                                    break;
                                }
                            }
                            if is_within_range {
                                current_location = temp;
                            }
                        }

                        current_location
                    })
                    .min()
                    .unwrap_or(u128::MAX)
            } else {
                u128::MAX
            }
        })
        .collect();

    let min_location_for_seed_pairs = min_locations.into_par_iter().min().unwrap_or(u128::MAX);

    (min_location, min_location_for_seed_pairs)
}

fn parse_tuple_lines<I, S>(lines: I) -> Vec<(u128, u128, u128)>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    lines
        .flat_map(|line| {
            let mut parts = line.as_ref().split_whitespace();
            let first = parts
                .next()
                .and_then(|s| s.parse::<u128>().ok())
                .unwrap_or_default();
            let second = parts
                .next()
                .and_then(|s| s.parse::<u128>().ok())
                .unwrap_or_default();
            let third = parts
                .next()
                .and_then(|s| s.parse::<u128>().ok())
                .unwrap_or_default();
            Some((first, second, third))
        })
        .collect()
}
