use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
struct HailStone {
    position: Vec<f64>,
    velocity: Vec<f64>,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    test_range(input, 200000000000000.0, 400000000000000.0)
}

fn test_range(input: &str, min: f64, max: f64) -> Result<usize> {
    let hailstones = aoc::parse_list::<String>(input)?
        .iter()
        .map(|line| {
            let line = line.replace(' ', "");
            let v = line
                .split('@')
                .map(|s| {
                    s.split(',')
                        .map(|n| n.parse::<f64>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            HailStone {
                position: v[0].to_vec(),
                velocity: v[1].to_vec(),
            }
        })
        .collect_vec();

    let mut answer = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i) {
            if let Some((x, y, t1, t2)) = calculate_intersection(h1, h2) {
                if x >= min && x <= max && y >= min && y <= max && t1 >= 0.0 && t2 >= 0.0 {
                    answer += 1;
                }
            }
        }
    }

    Ok(answer)
}

fn calculate_intersection(h1: &HailStone, h2: &HailStone) -> Option<(f64, f64, f64, f64)> {
    let p1 = &h1.position;
    let p2 = &h2.position;
    let v1 = &h1.velocity;
    let v2 = &h2.velocity;
    let m1 = v1[1] / v1[0];
    let m2 = v2[1] / v2[0];
    let b1 = p1[1] - m1 * p1[0];
    let b2 = p2[1] - m2 * p2[0];

    if m1 == m2 {
        return None;
    }

    let x = (b2 - b1) / (m1 - m2);
    let y = m1 * x + b1;
    let t1 = (x - p1[0]) / v1[0];
    let t2 = (x - p2[0]) / v2[0];
    Some((x, y, t1, t2))
}

fn tests() -> anyhow::Result<()> {
    let input = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    let solution = test_range(input, 7.0, 27.0)?;

    assert_eq!(solution, 2);

    Ok(())
}
