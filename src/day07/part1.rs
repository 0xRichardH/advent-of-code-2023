use std::{cmp::Ordering, collections::HashMap};

use anyhow::anyhow;
use nom::{
    character::complete::{self, alphanumeric1, multispace0, space1},
    multi::many1,
    sequence::{terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from(c: char) -> Option<Self> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }

    fn rank(&self) -> u8 {
        match self {
            Card::A => 13,
            Card::K => 12,
            Card::Q => 11,
            Card::J => 10,
            Card::T => 9,
            Card::Nine => 8,
            Card::Eight => 7,
            Card::Seven => 6,
            Card::Six => 5,
            Card::Five => 4,
            Card::Four => 3,
            Card::Three => 2,
            Card::Two => 1,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

#[derive(Debug)]
enum HandsType {
    FiveOfKind,  // AAAAA
    FourOfKind,  // AA8AA
    FullHouse,   // 23332
    ThreeOfKind, // TTT98
    TwoPair,     // 23432
    OnePair,     // A23A4
    HighCard,    // 23456
    Invalid,
}

impl Eq for HandsType {}

impl PartialEq for HandsType {
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}

impl HandsType {
    fn rank(&self) -> u8 {
        match self {
            HandsType::FiveOfKind => 7,
            HandsType::FourOfKind => 6,
            HandsType::FullHouse => 5,
            HandsType::ThreeOfKind => 4,
            HandsType::TwoPair => 3,
            HandsType::OnePair => 2,
            HandsType::HighCard => 1,
            HandsType::Invalid => u8::MIN,
        }
    }
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
        match self.kind.rank().cmp(&other.kind.rank()) {
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
        let mut frequency: HashMap<u8, usize> = HashMap::new();
        for card in cards.iter() {
            frequency
                .entry(card.rank())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let mut pattern = frequency.values().cloned().collect::<Vec<usize>>();
        pattern.sort();
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
        assert_eq!(6440, result);
    }
}
