use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Range {
    min: usize,
    max: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Brick {
    ordinal: usize,
    name: char,
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let lines = aoc::parse_list::<String>(input)?;
    let mut i = 0u8;
    let mut ordinal = 0;
    let mut bricks = lines
        .iter()
        .map(|line| {
            let points = line
                .split('~')
                .map(|coords| {
                    coords
                        .split(',')
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            i += 1;
            ordinal += 1;
            Brick {
                ordinal,
                name: (b'A' + i - 1) as char,
                x_range: Range {
                    min: points[0][0],
                    max: points[1][0],
                },
                y_range: Range {
                    min: points[0][1],
                    max: points[1][1],
                },
                z_range: Range {
                    min: points[0][2],
                    max: points[1][2],
                },
            }
        })
        .collect_vec();

    // Build a map of which spaces are occupied
    let mut occupied: HashMap<(usize, usize, usize), Brick> = HashMap::new();
    for brick in &bricks {
        for x in brick.x_range.min..=brick.x_range.max {
            for y in brick.y_range.min..=brick.y_range.max {
                for z in brick.z_range.min..=brick.z_range.max {
                    occupied.insert((x, y, z), brick.clone());
                }
            }
        }
    }

    // While we can lower bricks, lower them one at a time
    loop {
        let mut changes = 0;

        for brick in bricks.iter_mut() {
            // Continue if the brick is at the lowest level
            if brick.z_range.min == 1 {
                continue;
            }

            let mut can_descend = true;
            for x in brick.x_range.min..=brick.x_range.max {
                for y in brick.y_range.min..=brick.y_range.max {
                    if occupied.contains_key(&(x, y, brick.z_range.min - 1)) {
                        can_descend = false;
                        break;
                    }
                }
            }

            if can_descend {
                changes += 1;

                for x in brick.x_range.min..=brick.x_range.max {
                    for y in brick.y_range.min..=brick.y_range.max {
                        for z in brick.z_range.min..=brick.z_range.max {
                            occupied.remove(&(x, y, z));
                        }
                    }
                }

                brick.z_range.max -= 1;
                brick.z_range.min -= 1;

                for x in brick.x_range.min..=brick.x_range.max {
                    for y in brick.y_range.min..=brick.y_range.max {
                        for z in brick.z_range.min..=brick.z_range.max {
                            occupied.insert((x, y, z), brick.clone());
                        }
                    }
                }
            }
        }

        // println!("Changes: {changes}");
        if changes == 0 {
            break;
        }
    }

    let mut answer = 0;
    for brick in &bricks {
        if !can_be_disintegrated(brick, &occupied) {
            answer += disintegrate_brick(brick, &bricks, &occupied);
        }
    }

    Ok(answer)
}

fn disintegrate_brick(
    brick: &Brick,
    bricks: &[Brick],
    occupied: &HashMap<(usize, usize, usize), Brick>,
) -> usize {
    let mut occupied = occupied.clone();
    let mut bricks = bricks.iter().filter(|b| *b != brick).cloned().collect_vec();

    // First remove the brick from the occupied map
    for x in brick.x_range.min..=brick.x_range.max {
        for y in brick.y_range.min..=brick.y_range.max {
            for z in brick.z_range.min..=brick.z_range.max {
                occupied.remove(&(x, y, z));
            }
        }
    }

    // Now find out how many bricks can descend
    let mut impacted_bricks: HashSet<usize> = HashSet::new();
    loop {
        let mut changes = 0;

        for brick in bricks.iter_mut() {
            // Continue if the brick is at the lowest level
            if brick.z_range.min == 1 {
                continue;
            }

            let mut can_descend = true;
            for x in brick.x_range.min..=brick.x_range.max {
                for y in brick.y_range.min..=brick.y_range.max {
                    if occupied.contains_key(&(x, y, brick.z_range.min - 1)) {
                        can_descend = false;
                        break;
                    }
                }
            }

            if can_descend {
                impacted_bricks.insert(brick.ordinal);
                changes += 1;

                for x in brick.x_range.min..=brick.x_range.max {
                    for y in brick.y_range.min..=brick.y_range.max {
                        for z in brick.z_range.min..=brick.z_range.max {
                            occupied.remove(&(x, y, z));
                        }
                    }
                }

                brick.z_range.max -= 1;
                brick.z_range.min -= 1;

                for x in brick.x_range.min..=brick.x_range.max {
                    for y in brick.y_range.min..=brick.y_range.max {
                        for z in brick.z_range.min..=brick.z_range.max {
                            occupied.insert((x, y, z), brick.clone());
                        }
                    }
                }
            }
        }

        if changes == 0 {
            break;
        }
    }

    impacted_bricks.len()
}

fn can_be_disintegrated(brick: &Brick, occupied: &HashMap<(usize, usize, usize), Brick>) -> bool {
    // Sanity check
    for x in brick.x_range.min..=brick.x_range.max {
        for y in brick.y_range.min..=brick.y_range.max {
            for z in brick.z_range.min..=brick.z_range.max {
                if occupied.get(&(x, y, z)).unwrap().name != brick.name {
                    panic!("Didn't find {} in occupied list", brick.name);
                }
            }
        }
    }

    for x in brick.x_range.min..=brick.x_range.max {
        for y in brick.y_range.min..=brick.y_range.max {
            if let Some(supported_brick) = occupied.get(&(x, y, brick.z_range.max + 1)) {
                // We are supporting a brick.  Check to see if it has another support
                let mut has_another_support = false;

                'SUPPORT_SEARCH: for x in supported_brick.x_range.min..=supported_brick.x_range.max
                {
                    for y in supported_brick.y_range.min..=supported_brick.y_range.max {
                        if let Some(support) =
                            occupied.get(&(x, y, supported_brick.z_range.min - 1))
                        {
                            if support != brick {
                                has_another_support = true;
                                break 'SUPPORT_SEARCH;
                            }
                        }
                    }
                }

                if !has_another_support {
                    return false;
                }
            }
        }
    }

    true
}

fn tests() -> anyhow::Result<()> {
    let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    let solution = solve(input)?;

    assert_eq!(solution, 7);

    Ok(())
}
