use anyhow::Result;

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let lines = aoc::parse_list::<String>(input)?;

    let time: u64 = lines[0]
        .split(':')
        .skip(1)
        .map(|x| {
            x.split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>()[0];

    let distance: u64 = lines[1]
        .split(':')
        .skip(1)
        .map(|x| {
            x.split_whitespace()
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>()[0];

    // let mut answer = 1;
    let answer = ways_to_win(&time, &distance);

    // for i in 0..times.len() {
    //     answer = answer * ways_to_win(&times[i], &distances[i]);
    // }

    println!("{:?}, {:?}", time, distance);

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

    assert_eq!(solution, 71503);
    Ok(())
}
