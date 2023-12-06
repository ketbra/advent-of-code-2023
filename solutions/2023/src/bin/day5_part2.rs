use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct Range {
    src_start: u64,
    size: u64,
    dest_start: u64,
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    size: u64,
}

// struct CompressedCoordinates {
//     orig: HashSet<u64>,
//     to_compressed: HashMap<u64, u64>,
//     to_orig: HashMap<u64, u64>,
// }

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let lines = aoc::parse_list::<String>(input)?;

    let seeds: Vec<_> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let seed_ranges: Vec<SeedRange> = seeds
        .iter()
        .tuples()
        .map(|(start, end)| SeedRange {
            start: *start,
            size: *end,
        })
        .collect();

    println!("{:?}", seed_ranges);

    let mut i = 2;
    let seed_to_soil_map: Vec<Range> = parse_map(&lines, &mut i);
    let soil_to_fertilizer_map: Vec<Range> = parse_map(&lines, &mut i);
    let fertilizer_to_water_map: Vec<Range> = parse_map(&lines, &mut i);
    let water_to_light_map: Vec<Range> = parse_map(&lines, &mut i);
    let light_to_temperature_map: Vec<Range> = parse_map(&lines, &mut i);
    let temperature_to_humidity_map: Vec<Range> = parse_map(&lines, &mut i);
    let humidity_to_location_map: Vec<Range> = parse_map(&lines, &mut i);

    // Maybe compress coordinates to calculate it quickly
    // let mut c: Vec<u64> = Vec::new();
    // c.extend(seeds.iter().copied());
    // c.extend(seed_to_soil_map.iter().copied());

    // // let seed_coord = compress_coordinates();

    let mut min_location = None;
    for seed_range in seed_ranges {
        println!("Range {} of size {}", seed_range.start, seed_range.size);
        let start = seed_range.start;
        let end = seed_range.size + seed_range.start;
        let location = (start..end)
            .into_par_iter()
            .map(|seed| {
                get_location(
                    &seed,
                    &seed_to_soil_map,
                    &soil_to_fertilizer_map,
                    &fertilizer_to_water_map,
                    &water_to_light_map,
                    &light_to_temperature_map,
                    &temperature_to_humidity_map,
                    &humidity_to_location_map,
                )
            })
            .min()
            .unwrap();

        if min_location.is_none() || location < min_location.unwrap() {
            min_location = Some(location);
        }
    }

    let answer = min_location.unwrap();
    Ok(answer)
}
/*
fn compress_coordinates(orig: Vec<u64>) -> CompressedCoordinates {
    let mut set: HashSet<u64> = HashSet::new();
    for x in orig {
        set.insert(x);
    }

    let sorted_orig: Vec<_> = set.iter().sorted().collect();
    let mut to_compressed: HashMap<u64, u64> = HashMap::new();
    let mut to_orig: HashMap<u64, u64> = HashMap::new();
    for (i, val) in sorted_orig.into_iter().enumerate() {
        to_compressed.insert(*val, i as u64);
        to_orig.insert(i as u64, *val);
    }

    CompressedCoordinates {
        orig: set,
        to_compressed,
        to_orig,
    }
}
*/
fn get_location(
    seed: &u64,
    seed_to_soil_map: &Vec<Range>,
    soil_to_fertilizer_map: &Vec<Range>,
    fertilizer_to_water_map: &Vec<Range>,
    water_to_light_map: &Vec<Range>,
    light_to_temperature_map: &Vec<Range>,
    temperature_to_humidity_map: &Vec<Range>,
    humidity_to_location_map: &Vec<Range>,
) -> u64 {
    // Map to soil
    let soil = lookup(seed, seed_to_soil_map);

    // Map to fertilizer
    let fertilizer = lookup(&soil, soil_to_fertilizer_map);

    // Map to water
    let water = lookup(&fertilizer, fertilizer_to_water_map);

    // Map to light
    let light = lookup(&water, water_to_light_map);

    // Map to temperature
    let temperature = lookup(&light, light_to_temperature_map);

    // Map to humidity
    let humidity = lookup(&temperature, temperature_to_humidity_map);

    // Map to location
    lookup(&humidity, humidity_to_location_map)
}

fn lookup(x: &u64, ranges: &Vec<Range>) -> u64 {
    for range in ranges {
        if *x >= range.src_start && *x < range.src_start + range.size {
            return range.dest_start + x - range.src_start;
        }
    }

    *x
}

fn parse_map(lines: &Vec<String>, i: &mut usize) -> Vec<Range> {
    let mut ranges = Vec::new();

    // Skip the header line
    *i += 1;

    while *i < lines.len() && !lines[*i].is_empty() {
        let nums: Vec<_> = lines[*i]
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        ranges.push(Range {
            dest_start: nums[0],
            src_start: nums[1],
            size: nums[2],
        });
        *i += 1;
    }
    *i += 1;

    ranges
}

fn tests() -> anyhow::Result<()> {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    let solution = solve(input)?;

    assert_eq!(solution, 46);
    Ok(())
}
