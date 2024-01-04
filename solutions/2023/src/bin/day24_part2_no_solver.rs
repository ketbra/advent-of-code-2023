use anyhow::Result;
use itertools::Itertools;
use ndarray::*;
use ndarray_linalg::*;

// Note.  If we don't use z3::ast::Ast, then the _eq method is not available

#[derive(Debug, Clone, PartialEq)]
struct HailStone {
    position: Vec<f64>,
    velocity: Vec<f64>,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
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

    // Since there are only 6 unknowns, we only need to look at 3
    // hailstones.  This is because each set of 3 equations only
    // introduces 1 new unknown, the time variable.  The net effect of
    // each hailstone is to add two more equations than unknowns.
    // With 3 hailstones we have 9 equations which is enough to
    // resolve the 6 unknowns for the position and velocity of our
    // thrown hailstone as well as the 3 introduced time variables.
    // For convenience we use more than 3 hailstones since we have them.
    let mut a = Array2::<f64>::default((4, 4));
    let mut b = Array1::<f64>::default(4);
    for (i, (h1, h2)) in hailstones.iter().tuple_windows().take(4).enumerate() {
        a[[i, 0]] = h1.velocity[1] - h2.velocity[1];
        a[[i, 1]] = h2.velocity[0] - h1.velocity[0];
        a[[i, 2]] = h2.position[1] - h1.position[1];
        a[[i, 3]] = h1.position[0] - h2.position[0];

        b[i] = h1.position[0] * h1.velocity[1]
            - h1.position[1] * h1.velocity[0]
            - h2.position[0] * h2.velocity[1]
            + h2.position[1] * h2.velocity[0];
    }

    let x = a.solve_into(b).unwrap();
    let mut answer = 0;
    answer += x[0].round() as usize; // x0
    answer += x[1].round() as usize; // y0

    // Repeat to find z0 position
    let mut a = Array2::<f64>::default((4, 4));
    let mut b = Array1::<f64>::default(4);
    for (i, (h1, h2)) in hailstones.iter().tuple_windows().take(4).enumerate() {
        a[[i, 0]] = h1.velocity[1] - h2.velocity[1];
        a[[i, 1]] = h2.velocity[2] - h1.velocity[2];
        a[[i, 2]] = h2.position[1] - h1.position[1];
        a[[i, 3]] = h1.position[2] - h2.position[2];

        b[i] = h1.position[2] * h1.velocity[1]
            - h1.position[1] * h1.velocity[2]
            - h2.position[2] * h2.velocity[1]
            + h2.position[1] * h2.velocity[2];
    }
    let x = a.solve_into(b).unwrap();
    answer += x[0].round() as usize; // z0

    Ok(answer)
}

fn tests() -> anyhow::Result<()> {
    let input = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    let solution = solve(input)?;

    assert_eq!(solution, 47);

    Ok(())
}
