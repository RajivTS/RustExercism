use std::{cmp::Ordering, ops::Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

impl From<char> for Suit {
    fn from(input: char) -> Self {
        match input {
            'H' => Self::Heart,
            'S' => Self::Spade,
            'D' => Self::Diamond,
            'C' => Self::Club,
            _ => panic!("Invalid suit {}", input),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Number {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Number {
    fn from(input: char) -> Self {
        match input {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            '1' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid number {}", input),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Card(Suit, Number);

impl Card {
    fn new(input: &str) -> Self {
        let mut chars = input.chars();
        match (chars.next(), chars.next(), chars.next()) {
            (Some(number), Some(suit), None) => Self(suit.into(), number.into()),
            (Some(number), Some(_), Some(suit)) => Self(suit.into(), number.into()),
            _ => panic!("Incorrect card format {}", input),
        }
    }
}

impl Sub<&Card> for &Card {
    type Output = i32;

    fn sub(self, rhs: &Card) -> Self::Output {
        match (self.1, rhs.1) {
            (Number::Two, Number::Ace) => 1,
            (left, right) => (left as i32 - right as i32).abs(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    StraightFiveHigh,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlushFiveHigh,
    StraightFlush,
}

impl From<&[Card]> for Category {
    fn from(cards: &[Card]) -> Self {
        let number_freq = cards_number_freq(cards);
        let suit_frequency = cards_suit_freq(cards);
        match cards {
            [first, second, third, fourth, fifth] => {
                if is_consecutive_diff_one(first, second, third, fourth, fifth)
                    && suit_frequency.len() == 1
                {
                    match is_five_high(first, second, third, fourth, fifth) {
                        true => Self::StraightFlushFiveHigh,
                        false => Self::StraightFlush,
                    }
                } else if number_freq.len() == 2 {
                    if number_freq.iter().any(|&(_, s)| s == 4) {
                        Self::FourOfAKind
                    } else {
                        Self::FullHouse
                    }
                } else if suit_frequency.len() == 1 {
                    Self::Flush
                } else if is_consecutive_diff_one(first, second, third, fourth, fifth) {
                    match is_five_high(first, second, third, fourth, fifth) {
                        true => Self::StraightFiveHigh,
                        false => Self::Straight,
                    }
                } else if number_freq.iter().any(|&(_, s)| s == 3) {
                    Self::ThreeOfAKind
                } else {
                    match number_freq.iter().filter(|&&(_, s)| s == 2).count() {
                        2 => Self::TwoPair,
                        1 => Self::OnePair,
                        _ => Self::HighCard,
                    }
                }
            }
            _ => panic!("Invalid hand of cards {:?}", cards),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    original_hand: &'a str,
    cards: [Card; 5],
    category: Category,
}

impl<'a> Hand<'a> {
    fn new(original_hand: &'a str) -> Self {
        let mut cards: [Card; 5] = original_hand
            .split_whitespace()
            .map(Card::new)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        cards.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)).reverse());
        let category = cards.as_ref().into();
        Self {
            original_hand,
            cards,
            category,
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (self_number_freq, other_number_freq) = (
            cards_number_freq(&self.cards),
            cards_number_freq(&other.cards),
        );
        let ord = match self.category.cmp(&other.category) {
            Ordering::Equal => match self.category {
                Category::StraightFlush | Category::Straight | Category::StraightFlushFiveHigh | Category::StraightFiveHigh => {
                    self.cards[0].1.cmp(&other.cards[0].1)
                }
                Category::FourOfAKind | Category::FullHouse => {
                    compare_cards_in_order(&self_number_freq, &other_number_freq, 2)
                }
                Category::ThreeOfAKind | Category::TwoPair => {
                    compare_cards_in_order(&self_number_freq, &other_number_freq, 3)
                }
                Category::OnePair => {
                    compare_cards_in_order(&self_number_freq, &other_number_freq, 4)
                }
                Category::Flush | Category::HighCard => {
                    compare_cards_in_order(&self_number_freq, &other_number_freq, 5)
                }
            },
            ord => ord,
        };
        Some(ord)
    }
}

fn is_consecutive_diff_one(a: &Card, b: &Card, c: &Card, d: &Card, e: &Card) -> bool {
    match ((a - b), (b - c), (c - d), (d - e)) {
        (1, 1, 1, 1) | (9, 1, 1, 1) => true,
        _ => false,
    }
}

fn is_five_high(a: &Card, b: &Card, c: &Card, d: &Card, e: &Card) -> bool {
    match ((a - b), (b - c), (c - d), (d - e)) {
        (9, 1, 1, 1) => true,
        _ => false,
    }
}

fn compare_cards_in_order(
    self_cards: &Vec<(Number, usize)>,
    other_cards: &Vec<(Number, usize)>,
    len: usize,
) -> Ordering {
    let mut ord = self_cards
        .get(0)
        .unwrap()
        .0
        .cmp(&other_cards.get(0).unwrap().0);
    for idx in 1..len {
        ord = ord.then(
            self_cards
                .get(idx)
                .unwrap()
                .0
                .cmp(&other_cards.get(idx).unwrap().0),
        );
    }
    ord
}

fn cards_number_freq(cards: &[Card]) -> Vec<(Number, usize)> {
    let mut cards_vec = cards
        .iter()
        .map(|c| (c.1, cards.iter().filter(|ca| ca.1 == c.1).count()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    cards_vec.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)).reverse());
    cards_vec
}

fn cards_suit_freq(cards: &[Card]) -> Vec<(Suit, usize)> {
    cards
        .iter()
        .map(|c| (c.0, cards.iter().filter(|ca| ca.0 == c.0).count()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut parsed_hands = hands.iter().map(|&h| Hand::new(h)).collect::<Vec<_>>();
    parsed_hands.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal).reverse());
    dbg!(&parsed_hands);
    let mut result: Vec<&'a str> = Vec::new();
    if let Some(compare) = parsed_hands.first() {
        result.extend(parsed_hands.iter().filter_map(|h| {
            if compare.partial_cmp(h).unwrap_or(Ordering::Equal) == Ordering::Equal {
                Some(h.original_hand)
            } else {
                None
            }
        }))
    }
    result
}
