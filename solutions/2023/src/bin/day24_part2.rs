use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use z3::ast::Real;
use z3::{Config, Context, Solver};

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

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let x_0 = Real::fresh_const(&ctx, "x__0");
    let y_0 = Real::fresh_const(&ctx, "y__0");
    let z_0 = Real::fresh_const(&ctx, "z__0");
    let vx_0 = Real::fresh_const(&ctx, "vx__0");
    let vy_0 = Real::fresh_const(&ctx, "vy__0");
    let vz_0 = Real::fresh_const(&ctx, "vz__0");

    let ts = hailstones
        .iter()
        .enumerate()
        .map(|(i, _)| Real::fresh_const(&ctx, &format!("t__{}", i + 1)))
        .collect_vec();

    for (i, h) in hailstones.iter().enumerate() {
        // solver.assert(x_0 + vx_0 * ts[i] == h.position[0] + h.velocity[0] * ts[i]);
        solver.assert(&x_0 + &vx_0 == 5);
    }
    // ctx.
    // let x_0 = ctx.real_const("x__0");
    // let y_0 = ctx.real_const("y__0");
    // let z_0 = ctx.real_const("z__0");
    // let vx_0 = ctx.real_const("vx__0");
    // let vy_0 = ctx.real_const("vy__0");
    // let vz_0 = ctx.real_const("vz__0");

    // let mut answer = 0;
    // let throw = HailStone {
    //     position: vec![
    //         390970075767404.0,
    //         226195425713131238783606819606.0,
    //         466980968436673869995870521057.0,
    //     ],
    //     velocity: vec![
    //         -147.0,
    //         59.0 * 226195425713131238783606819606.0,
    //         -107.0 * 466980968436673869995870521057.0,
    //     ],
    // };
    // for h1 in &hailstones {
    //     if let Some((x, y, t1, t2)) = calculate_intersection(h1, &throw) {
    //         println!("{t1},{t2}");
    //         if x >= min && x <= max && y >= min && y <= max && t1 >= 0.0 && t2 >= 0.0 {
    //             answer += 1;
    //         }
    //     }
    // }

    let answer = 4;
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
