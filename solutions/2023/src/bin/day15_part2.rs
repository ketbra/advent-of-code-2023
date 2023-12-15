use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Default)]
struct Entry {
    label: String,
    focal_length: usize,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let input = input.replace('\n', "");
    let inputs = input.split(',').collect_vec();

    let mut boxes: Vec<Vec<Entry>> = std::iter::repeat(vec![]).take(256).collect::<Vec<_>>();

    'INPUT: for input in inputs {
        if input.ends_with('-') {
            let label = &input[0..(input.len() - 1)];

            // Remove the entry
            let vec = &mut boxes[calculate_hash(label)];
            for (i, entry) in vec.iter_mut().enumerate() {
                if entry.label == label {
                    vec.remove(i);
                    continue 'INPUT;
                }
            }
        } else {
            let parts = input.split('=').collect_vec();
            let label = parts[0].to_string();
            let focal_length = parts[1].parse::<usize>().unwrap();

            let vec = &mut boxes[calculate_hash(&label)];
            for entry in vec.iter_mut() {
                if entry.label == label {
                    entry.focal_length = focal_length;
                    continue 'INPUT;
                }
            }

            vec.push(Entry {
                label,
                focal_length,
            });

            // println!("{:?}", parts);
        }
        // println!("{:?}", parts);

        // answer += calculate_hash(input);
    }

    let mut answer = 0;

    for (i, b) in boxes.iter().enumerate() {
        for (j, entry) in b.iter().enumerate() {
            answer += (1 + i) * (j + 1) * entry.focal_length;
        }
    }

    Ok(answer)
}

fn calculate_hash(s: &str) -> usize {
    let mut hash = 0;

    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn tests() -> anyhow::Result<()> {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    let solution = solve(input)?;

    assert_eq!(solution, 145);

    Ok(())
}
