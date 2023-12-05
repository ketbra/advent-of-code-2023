use anyhow::Result;

#[derive(Debug)]
struct Range {
    src_start: u64,
    size: u64,
    dest_start: u64,
}

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let lines = aoc::parse_list::<String>(input)?;

    let seeds: Vec<_> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut i = 2;
    let seed_to_soil_map: Vec<Range> = parse_map(&lines, &mut i);
    let soil_to_fertilizer_map: Vec<Range> = parse_map(&lines, &mut i);
    let fertilizer_to_water_map: Vec<Range> = parse_map(&lines, &mut i);
    let water_to_light_map: Vec<Range> = parse_map(&lines, &mut i);
    let light_to_temperature_map: Vec<Range> = parse_map(&lines, &mut i);
    let temperature_to_humidity_map: Vec<Range> = parse_map(&lines, &mut i);
    let humidity_to_location_map: Vec<Range> = parse_map(&lines, &mut i);

    let mut min_location = None;

    for seed in &seeds {
        // Map to soil
        let soil = lookup(seed, &seed_to_soil_map);

        // Map to fertilizer
        let fertilizer = lookup(&soil, &soil_to_fertilizer_map);

        // Map to water
        let water = lookup(&fertilizer, &fertilizer_to_water_map);

        // Map to light
        let light = lookup(&water, &water_to_light_map);

        // Map to temperature
        let temperature = lookup(&light, &light_to_temperature_map);

        // Map to humidity
        let humidity = lookup(&temperature, &temperature_to_humidity_map);

        // Map to location
        let location = lookup(&humidity, &humidity_to_location_map);

        if min_location.is_none() || location < min_location.unwrap() {
            min_location = Some(location);
        }
    }

    println!("{:?}", seeds);
    println!("{:?}", seed_to_soil_map);
    println!("{:?}", humidity_to_location_map);

    Ok(min_location.unwrap())
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

    assert_eq!(solution, 35);
    Ok(())
}
