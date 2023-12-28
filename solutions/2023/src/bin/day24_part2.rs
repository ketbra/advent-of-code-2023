use anyhow::Result;
use itertools::Itertools;
use z3::ast::{Ast, Int, Real};

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

    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);

    // Since there are only 6 unknowns, we only need to look at 3
    // hailstones.  This is because each set of 3 equations only
    // introduces 1 new unknown, the time variable.  The net effect of
    // each hailstone is to add two more equations than unknowns.
    // With 3 hailstones we have 9 equations which is enough to
    // resolve the 6 unknowns for the position and velocity of our
    // thrown hailstone as well as the 3 introduced time variables.
    let hailstones = hailstones.iter().take(3).collect_vec();

    // Create two real variables
    let x_0 = Real::new_const(&ctx, "x__0");
    let y_0 = Real::new_const(&ctx, "y__0");
    let z_0 = Real::new_const(&ctx, "z__0");
    let vx_0 = Real::new_const(&ctx, "vx__0");
    let vy_0 = Real::new_const(&ctx, "vy__0");
    let vz_0 = Real::new_const(&ctx, "vz__0");
    let ts = hailstones
        .iter()
        .enumerate()
        .map(|(i, _)| Real::new_const(&ctx, format!("t__{}", i + 1)))
        .collect_vec();

    for (i, h) in hailstones.iter().enumerate() {
        let p = h
            .position
            .iter()
            .map(|x| Int::from_i64(&ctx, *x as i64).to_real())
            .collect_vec();

        let v = h
            .velocity
            .iter()
            .map(|x| Int::from_i64(&ctx, *x as i64).to_real())
            .collect_vec();

        s.assert(&((&x_0 + &vx_0 * &ts[i])._eq(&(&p[0] + &v[0] * &ts[i]))));
        s.assert(&((&y_0 + &vy_0 * &ts[i])._eq(&(&p[1] + &v[1] * &ts[i]))));
        s.assert(&((&z_0 + &vz_0 * &ts[i])._eq(&(&p[2] + &v[2] * &ts[i]))));
    }

    // Check if the solver is satisfiable
    let mut answer = 0;
    if s.check() == z3::SatResult::Sat {
        // Get the model
        let model = s.get_model().unwrap();

        // Get the values of x and y from the model
        let (x_value, x_denominator) = model.eval(&x_0, true).unwrap().as_real().unwrap();
        let (y_value, y_denominator) = model.eval(&y_0, true).unwrap().as_real().unwrap();
        let (z_value, z_denominator) = model.eval(&z_0, true).unwrap().as_real().unwrap();

        // Make sure the solution values are integers
        assert_eq!(x_denominator, 1);
        assert_eq!(y_denominator, 1);
        assert_eq!(z_denominator, 1);

        answer = (x_value + y_value + z_value) as usize;
    } else {
        println!("No solution found");
    }

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
