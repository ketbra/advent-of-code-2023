use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    rank: u64,
    bid: u64,
}

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let lines = aoc::parse_list::<String>(input)?;

    let card_weights: HashMap<char, usize> = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .into_iter()
    .rev()
    .enumerate()
    .map(|(i, char)| (char, i))
    .collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let v: Vec<_> = line.split_whitespace().collect();
        let cards = v[0].chars().collect_vec();
        let bid = v[1].parse::<u64>().unwrap();
        let rank = hand_rank(&cards);

        hands.push(Hand { cards, bid, rank });
    }

    hands.sort_unstable_by(|a, b| match a.rank.cmp(&b.rank) {
        std::cmp::Ordering::Equal => {
            for i in 0..5 {
                if card_weights[&a.cards[i]] != card_weights[&b.cards[i]] {
                    return card_weights[&a.cards[i]].cmp(&card_weights[&b.cards[i]]);
                }
            }
            std::cmp::Ordering::Equal
        }
        x => x,
    });

    let mut answer = 0;
    for (i, hand) in hands.iter().enumerate() {
        answer += hand.bid * (i as u64 + 1);
    }

    println!("{:?}", hands);

    Ok(answer)
}

fn hand_rank(cards: &Vec<char>) -> u64 {
    let mut map: HashMap<char, u8> = HashMap::new();

    let mut sorted_cards = cards.clone();
    sorted_cards.sort();

    for card in cards {
        if map.contains_key(card) {
            map.insert(*card, map[card] + 1);
        } else {
            map.insert(*card, 1);
        }
    }

    let mut has_pair = false;
    let mut has_2_pair = false;
    let mut has_triple = false;
    let mut has_4 = false;
    let mut has_5 = false;

    map.values().for_each(|val| {
        if *val == 5 {
            has_5 = true;
        } else if *val == 4 {
            has_4 = true;
        } else if *val == 3 {
            has_triple = true;
        } else if *val == 2 {
            if has_pair {
                has_2_pair = true;
            } else {
                has_pair = true;
            }
        }
    });

    if has_5 {
        return 6;
    }
    if has_4 {
        return 5;
    }
    if has_triple && has_pair {
        return 4;
    }
    if has_triple {
        return 3;
    }
    if has_2_pair {
        return 2;
    }
    if has_pair {
        return 1;
    }
    0
}

fn tests() -> anyhow::Result<()> {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    let solution = solve(input)?;

    assert_eq!(solution, 6440);
    Ok(())
}
