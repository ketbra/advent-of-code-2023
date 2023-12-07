use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    rank: u64,
    bid: u64,
}

enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let lines = aoc::parse_list::<String>(input)?;

    let card_weights: HashMap<char, usize> = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
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

    Ok(answer)
}

fn hand_rank(cards: &[char]) -> u64 {
    // Get joker count
    let joker_count = cards.iter().filter(|x| **x == 'J').count();

    if joker_count == 5 {
        return 6;
    }

    // Always makes sense to replace the second most popular card
    let no_jokers: Vec<_> = cards.iter().filter(|x| **x != 'J').collect();
    let mut map: HashMap<char, u8> = HashMap::new();
    for card in no_jokers {
        if map.contains_key(card) {
            map.insert(*card, map[card] + 1);
        } else {
            map.insert(*card, 1);
        }
    }

    let key_with_max_value = map.iter().max_by_key(|entry| entry.1).unwrap();
    map.insert(
        *key_with_max_value.0,
        map[key_with_max_value.0] + (joker_count as u8),
    );

    let mut hand_type = HandType::HighCard;
    map.values().for_each(|val| {
        if *val == 5 {
            hand_type = HandType::FiveOfAKind;
        } else if *val == 4 {
            hand_type = HandType::FourOfAKind;
        } else if *val == 3 {
            hand_type = match hand_type {
                HandType::Pair => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            };
        } else if *val == 2 {
            hand_type = match hand_type {
                HandType::ThreeOfAKind => HandType::FullHouse,
                HandType::Pair => HandType::TwoPair,
                _ => HandType::Pair,
            };
        }
    });

    hand_type as u64
}

fn tests() -> anyhow::Result<()> {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    let solution = solve(input)?;

    assert_eq!(solution, 5905);
    Ok(())
}
