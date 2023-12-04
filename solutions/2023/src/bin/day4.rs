use anyhow::Result;
use aoc::Searchable;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u64>,
    your_numbers: Vec<u64>,
}

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let mut cards = Vec::new();
    for line in aoc::parse_list::<String>(input)? {
        let (_, id, winning_numbers, your_numbers) = regex_captures!(
            r#"Card\s+(\d+)\s*:((?:\s+\d+)+)\s*\|\s*((?:\s+\d+)+)"#,
            &line
        )
        .unwrap();

        println!("{}, {}, {}", id, winning_numbers, your_numbers);

        let id: u32 = id.parse().unwrap();
        let winning_numbers: HashSet<u64> = winning_numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let your_numbers: Vec<u64> = your_numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        cards.push(Card {
            id,
            winning_numbers,
            your_numbers,
        });
    }

    let mut answer = 0;
    for card in cards {
        let winners = card
            .your_numbers
            .iter()
            .filter(|x| card.winning_numbers.contains(x))
            .count();
        if winners > 0 {
            let points: u64 = u64::pow(2, winners as u32 - 1);
            println!("Card {} is {} points", card.id, points);
            answer += points;
        }
    }

    // println!("{:?}", number_map);
    Ok(answer)
}

fn tests() -> anyhow::Result<()> {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    let solution = solve(input)?;

    assert_eq!(solution, 13);
    Ok(())
}
