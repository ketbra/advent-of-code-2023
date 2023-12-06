use anyhow::Result;

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let lines = aoc::parse_list::<String>(input)?;

    let times: Vec<_> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<_> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut answer = 1;
    for i in 0..times.len() {
        answer = answer * ways_to_win(&times[i], &distances[i]);
    }

    println!("{:?}, {:?}", times, distances);

    Ok(answer)
}

fn ways_to_win(time: &u64, distance: &u64) -> u64 {
    let mut wins = 0;
    for i in 1..*time {
        if (time - i) * i > *distance {
            wins += 1;
        }
    }
    wins
}

fn tests() -> anyhow::Result<()> {
    let input = "Time:      7  15   30
Distance:  9  40  200
";

    let solution = solve(input)?;

    assert_eq!(solution, 288);
    Ok(())
}
