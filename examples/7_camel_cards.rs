use std::char;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialOrd, Ord, Copy, PartialEq, Eq, Hash)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'J' => Self::Jack,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, Copy, PartialEq, Eq, Hash)]
enum Card2 {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<Card> for Card2 {
    fn from(c: Card) -> Self {
        match c {
            Card::Two => Self::Two,
            Card::Three => Self::Three,
            Card::Four => Self::Four,
            Card::Five => Self::Five,
            Card::Six => Self::Six,
            Card::Seven => Self::Seven,
            Card::Eight => Self::Eight,
            Card::Nine => Self::Nine,
            Card::Ten => Self::Ten,
            Card::Jack => Self::Joker,
            Card::Queen => Self::Queen,
            Card::King => Self::King,
            Card::Ace => Self::Ace,
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, Copy, PartialEq, Eq, Hash)]
enum HandType {
    HighCard = 1,
    Pair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl From<&[Card; 5]> for HandType {
    fn from(cards: &[Card; 5]) -> Self {
        let mut counts = HashMap::new();
        for card in cards.iter() {
            *counts.entry(*card).or_insert(0) += 1;
        }

        let mut max_count = 0;
        let mut second_max_count = 0;
        for count in counts.values() {
            if *count > max_count {
                second_max_count = max_count;
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }

        match max_count {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => match second_max_count {
                2 => Self::FullHouse,
                1 => Self::ThreeOfAKind,
                _ => panic!("invalid hand"),
            },
            2 => match second_max_count {
                2 => Self::TwoPairs,
                1 => Self::Pair,
                _ => panic!("invalid hand"),
            },
            1 => Self::HighCard,
            _ => panic!("invalid hand"),
        }
    }
}

impl From<&[Card2; 5]> for HandType {
    fn from(cards: &[Card2; 5]) -> Self {
        let mut counts = HashMap::new();
        for card in cards.iter() {
            *counts.entry(*card).or_insert(0) += 1;
        }

        // first get rid of jokers
        let n_jokers = counts.remove(&Card2::Joker).unwrap_or(0);

        let mut max_count = 0;
        let mut second_max_count = 0;
        for count in counts.values() {
            if *count > max_count {
                second_max_count = max_count;
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }
        // now consider jokers as the max card
        let max_count = max_count + n_jokers;

        match max_count {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => match second_max_count {
                2 => Self::FullHouse,
                1 => Self::ThreeOfAKind,
                _ => panic!("invalid hand"),
            },
            2 => match second_max_count {
                2 => Self::TwoPairs,
                1 => Self::Pair,
                _ => panic!("invalid hand"),
            },
            1 => Self::HighCard,
            _ => panic!("invalid hand"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    hand: HandType,
    bid: usize,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);

        let mut cards = [Card::Two; 5];
        let chars = parts[0].chars().collect::<Vec<_>>();
        for i in 0..5 {
            cards[i] = chars[i].into();
        }
        let hand = HandType::from(&cards);
        let bid = parts[1].parse().expect("cannot parse bid");

        Ok(Self { cards, hand, bid })
    }
}

#[derive(Debug)]
struct Hand2 {
    cards: [Card2; 5],
    hand: HandType,
    bid: usize,
}

impl From<&Hand> for Hand2 {
    fn from(hand: &Hand) -> Self {
        let mut cards = [Card2::Two; 5];
        for (i, card) in hand.cards.iter().enumerate() {
            cards[i] = (*card).into();
        }
        let hand_type = HandType::from(&cards);
        Self {
            cards,
            hand: hand_type,
            bid: hand.bid,
        }
    }
}

fn parse_input(path: &str) -> Vec<Hand> {
    let mut res = Vec::new();
    let lines = std::fs::read_to_string(path).expect("cannot read file");
    for line in lines.lines() {
        res.push(line.parse().expect("cannot parse line"));
    }
    res
}

fn exercise1(hands: &[Hand]) -> usize {
    let mut hand_types = HashMap::new();

    for hand in hands.iter() {
        hand_types.entry(hand.hand).or_insert(Vec::new()).push(hand);
    }

    let mut sorted = Vec::new();

    for hand in [
        HandType::HighCard,
        HandType::Pair,
        HandType::TwoPairs,
        HandType::ThreeOfAKind,
        HandType::FullHouse,
        HandType::FourOfAKind,
        HandType::FiveOfAKind,
    ] {
        if let Some(hands) = hand_types.get(&hand) {
            let mut sorted_hands = hands.to_vec();
            sorted_hands.sort_by(|a, b| a.cards.cmp(&b.cards));
            sorted.extend(sorted_hands);
        }
    }
    let mut res = 0;
    for (i, hand) in sorted.iter().enumerate() {
        let points = hand.bid * (i + 1);
        // println!("{}: {:?} {}", i, hand, points);
        res += points;
    }
    res
}

fn exercise2(hands: &[Hand2]) -> usize {
    let mut hand_types = HashMap::new();

    for hand in hands.iter() {
        hand_types.entry(hand.hand).or_insert(Vec::new()).push(hand);
    }

    let mut sorted = Vec::new();

    for hand in [
        HandType::HighCard,
        HandType::Pair,
        HandType::TwoPairs,
        HandType::ThreeOfAKind,
        HandType::FullHouse,
        HandType::FourOfAKind,
        HandType::FiveOfAKind,
    ] {
        if let Some(hands) = hand_types.get(&hand) {
            let mut sorted_hands = hands.to_vec();
            sorted_hands.sort_by(|a, b| a.cards.cmp(&b.cards));
            sorted.extend(sorted_hands);
        }
    }
    let mut res = 0;
    for (i, hand) in sorted.iter().enumerate() {
        let points = hand.bid * (i + 1);
        println!("{}: {:?} {}", i, hand, points);
        res += points;
    }
    res
}

fn main() {
    let hands = parse_input("data/7_input.txt");
    println!("exercise 1: {}", exercise1(&hands));

    let hands: Vec<Hand2> = hands.iter().map(|h| h.into()).collect();
    println!("exercise 2: {}", exercise2(&hands));
}
