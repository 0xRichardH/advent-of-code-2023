use std::{cmp::Ordering, collections::HashMap};

use anyhow::anyhow;
use nom::{
    character::complete::{self, alphanumeric1, multispace0, space1},
    multi::many1,
    sequence::{terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    A = 13,
    K = 12,
    Q = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    J = 1,
}

impl Card {
    fn from(c: char) -> Option<Self> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'T' => Some(Card::T),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            'J' => Some(Card::J),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandsType {
    FiveOfKind = 7,  // AAAAA
    FourOfKind = 6,  // AA8AA
    FullHouse = 5,   // 23332
    ThreeOfKind = 4, // TTT98
    TwoPair = 3,     // 23432
    OnePair = 2,     // A23A4
    HighCard = 1,    // 23456
    Invalid = 0,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    kind: HandsType,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare kind
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => (),
            other => return other,
        }

        // compare cards in ordering if kind is equal
        let pair = self.cards.iter().zip(other.cards.iter());
        for (a, b) in pair {
            match a.cmp(b) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        let mut frequency: HashMap<&Card, usize> = HashMap::new();
        for card in cards.iter() {
            frequency.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut j_count = 0;
        if let Some(c) = frequency.remove(&Card::J) {
            j_count = c;
        }

        let mut pattern = frequency.values().cloned().collect::<Vec<usize>>();
        pattern.sort();
        if j_count > 0 {
            let len = pattern.len();
            if len > 0 {
                pattern[len - 1] += j_count;
            } else {
                pattern.push(j_count);
            }
        }

        let kind = match pattern.as_slice() {
            [5] => HandsType::FiveOfKind,
            [1, 4] => HandsType::FourOfKind,
            [2, 3] => HandsType::FullHouse,
            [1, 1, 3] => HandsType::ThreeOfKind,
            [1, 2, 2] => HandsType::TwoPair,
            [1, 1, 1, 2] => HandsType::OnePair,
            [1, 1, 1, 1] => HandsType::HighCard,
            _ => HandsType::Invalid,
        };

        Self { cards, kind, bid }
    }
}

pub fn process_data(input: &str) -> anyhow::Result<u32> {
    let (_, mut hands) = parse_hands(input).map_err(|e| anyhow!("parse error: {:?}", e))?;
    hands.sort();
    let total_winning = hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, h)| acc + (idx as u32 + 1) * h.bid);

    Ok(total_winning)
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    many1(terminated(parse_hands_and_bids, multispace0)).parse(input)
}

fn parse_hands_and_bids(input: &str) -> IResult<&str, Hand> {
    let (input, (str_cards, _, bids)) =
        tuple((alphanumeric1, space1, complete::u32)).parse(input)?;
    let cards = str_cards
        .chars()
        .flat_map(Card::from)
        .collect::<Vec<Card>>();
    let hand = Hand::new(cards, bids);
    Ok((input, hand))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        let result = process_data(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
        .unwrap();
        assert_eq!(5905, result);
    }
}
