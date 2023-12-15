use core::panic;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    // Joker lowest
    J,
}

impl Card {
    fn from_char(char: char) -> Self {
        match char {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            'J' => Card::J,
            err => panic!("invalid char: {err}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Type {
    fn from_card_count(hand_count: &[(Card, usize)]) -> Type {
        if hand_count[0].1 == 5 {
            Type::FiveOfAKind
        } else if hand_count[0].1 == 4 {
            Type::FourOfAKind
        } else if hand_count[0].1 == 3 && hand_count[1].1 == 2 {
            if hand_count[1].0 == Card::J {
                panic!("Jokers can't be pairs");
            }
            Type::FullHouse
        } else if hand_count[0].1 == 3 {
            Type::ThreeOfAKind
        } else if hand_count[0].1 == 2 && hand_count[1].1 == 2 {
            if hand_count[1].0 == Card::J {
                panic!("Jokers can't be pairs");
            }
            Type::TwoPair
        } else if hand_count[0].1 == 2 {
            Type::OnePair
        } else if hand_count[0].1 == 1 {
            Type::HighCard
        } else {
            panic!("Unrecognized hand: {hand_count:?}")
        }
    }
}

fn count_matching_cards(hand: &[Card]) -> Vec<(Card, usize)> {
    let mut count = HashMap::with_capacity(5);
    for card in hand {
        count
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let num_jokers = *count.get(&Card::J).unwrap_or(&0);

    let mut count: Vec<_> = count
        .into_iter()
        .map(|(key, value)| (*key, value))
        .collect();
    count.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    // Find first non-joker and increment the number
    if num_jokers > 0 {
        for res in &mut count {
            if res.0 != Card::J {
                res.1 += num_jokers;
                break;
            }
        }

        // Sort again, shifting jokers to the end to prevent us building errornous two-pairs and full-houses.
        // For example, if the second pair are jokers then they are already in use building the first
        // which is an invalid hand.
        count.sort_unstable_by(|(a_card, a_num), (b_card, b_num)| {
            if a_card == &Card::J && b_card == &Card::J {
                std::cmp::Ordering::Equal
            } else if a_card == &Card::J {
                std::cmp::Ordering::Greater
            } else if b_card == &Card::J {
                std::cmp::Ordering::Less
            } else {
                b_num.cmp(a_num)
            }
        });
    }

    count
}

pub(crate) fn two(input: &[String]) {
    use itertools::Itertools;

    let now = std::time::Instant::now();

    let _iter = 0;

    let mut input: Vec<(Vec<Card>, usize, Type)> = input
        .iter()
        .map(|row| row.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(hand, bid): (&str, &str)| {
            (
                hand.chars().map(Card::from_char).collect::<Vec<_>>(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .map(|(hand, bet)| {
            let hand_type = Type::from_card_count(&count_matching_cards(&hand));
            (hand, bet, hand_type)
        })
        .collect();

    input.sort_by(|(a_hand, _, a_type), (b_hand, _, b_type)| {
        // Primary ordering, by type
        if a_type != b_type {
            return a_type.cmp(b_type);
        // Secondary ordering, highest card
        } else {
            for (a_card, b_card) in a_hand.iter().zip(b_hand.iter()) {
                if a_card != b_card {
                    return a_card.cmp(b_card);
                }
            }
        }
        // We end up here if we have to exactly equal hands, lets see if that happens.
        panic!("No ordering found");
    });

    let sum: usize = input
        .into_iter()
        .rev()
        .enumerate()
        .map(|(rank, (_, bet, _))| (rank + 1) * bet)
        .sum();

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}
