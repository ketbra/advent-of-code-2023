use anyhow::Result;
use aoc::Searchable;
use itertools::Itertools;

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let vals: Vec<String> = aoc::parse_list(input)?;

    println!("{:?}", vals);

    // Split up the lines into chars
    let lines: Vec<Vec<char>> = vals.iter().map(|line| line.chars().collect_vec()).collect();
    let mut answer = 0;
    (0..lines.len()).for_each(|i| {
        let line = &lines[i];
        // Walk each line.  Accumulate characters into the current number.
        // After hit blank or end of line, add number to sum if any are
        // next to a symbol
        let mut curr_num = "".to_owned();
        let mut curr_valid = false;
        (0..line.len()).for_each(|j| {
            if line[j].is_numeric() {
                curr_num.push(line[j]);
                if !curr_valid
                    && neighbors(i, j).iter().any(|(x, y)| {
                        if let Some(c) = lines.get(*x).and_then(|l| l.get(*y)) {
                            is_symbol(*c)
                        } else {
                            false
                        }
                    })
                {
                    curr_valid = true;
                }
            } else {
                if curr_valid {
                    println!("Parsing {}", curr_num);
                    answer += curr_num.parse::<u64>().unwrap();
                }
                curr_num = "".to_owned();
                curr_valid = false;
            }
        });
        if curr_valid {
            answer += curr_num.parse::<u64>().unwrap();
        }
    });
    Ok(answer)
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut xs = vec![(x, y + 1), (x + 1, y), (x + 1, y + 1)];
    if x > 0 {
        if y > 0 {
            xs.push((x - 1, y - 1));
        }
        xs.push((x - 1, y));
        xs.push((x - 1, y + 1));
    }
    if y > 0 {
        xs.push((x, y - 1));
        xs.push((x + 1, y - 1));
    }
    xs
}

fn tests() -> anyhow::Result<()> {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    let solution = solve(input)?;

    assert_eq!(solution, 4361);
    Ok(())
}
