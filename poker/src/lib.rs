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
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid number {}", input),
        }
    }
}

#[derive(Debug)]
struct Card(Suit, Number);

impl Card {
    fn new(input: &str) -> Self {
        let mut chars = input.chars();
        match (chars.next(), chars.next()) {
            (Some(number), Some(suit)) => Self(suit.into(), number.into()),
            _ => panic!("Incorrect card format {}", input),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Sub<&Card> for &Card {
    type Output = i32;

    fn sub(self, rhs: &Card) -> Self::Output {
        let diff = self.1 as i32 - rhs.1 as i32;
        diff.abs()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl From<&[Card]> for Category {
    fn from(cards: &[Card]) -> Self {
        let number_freq = cards_number_freq(cards);
        let suit_frequency = cards_suit_freq(cards);
        match cards {
            [first, second, third, fourth, fifth] => {
                if first - second == 1
                    && second - third == 1
                    && third - fourth == 1
                    && fourth - fifth == 1
                    && suit_frequency.len() == 1
                {
                    Self::StraightFlush
                } else if number_freq.len() == 2 {
                    if number_freq.iter().any(|&(_, s)| s == 4) {
                        Self::FourOfAKind
                    } else {
                        Self::FullHouse
                    }
                } else if suit_frequency.len() == 1 {
                    Self::Flush
                } else if first - second == 1
                    && second - third == 1
                    && third - fourth == 1
                    && fourth - fifth == 1
                {
                    Self::Straight
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

#[derive(PartialEq)]
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
        let suit_frequency = cards_suit_freq(&self.cards);
        let ord = match self.category.cmp(&other.category) {
            Ordering::Equal => match self.category {
                Category::StraightFlush | Category::Straight => self.cards[0].1.cmp(&other.cards[0].1),
                Category::FourOfAKind => {
                    let self_quad_num = number_with_freq(&self_number_freq, 4);
                    let self_single_num = number_with_freq(&self_number_freq, 1);
                    let other_quad_num = number_with_freq(&other_number_freq, 4);
                    let other_single_num = number_with_freq(&other_number_freq, 1);
                    self_quad_num
                        .0
                        .cmp(&other_quad_num.0)
                        .then(self_single_num.0.cmp(&other_single_num.0))
                }
                Category::FullHouse => {
                    let self_tri_num = number_with_freq(&self_number_freq, 3);
                    let self_twice_num = number_with_freq(&self_number_freq, 2);
                    let other_tri_num = number_with_freq(&other_number_freq, 3);
                    let other_twice_num = number_with_freq(&other_number_freq, 2);
                    self_tri_num
                        .0
                        .cmp(&other_tri_num.0)
                        .then(self_twice_num.0.cmp(&other_twice_num.0))
                }
                Category::Flush => self.cards[0]
                    .1
                    .cmp(&other.cards[0].1)
                    .then(self.cards[1].1.cmp(&other.cards[1].1))
                    .then(self.cards[2].1.cmp(&other.cards[2].1))
                    .then(self.cards[3].1.cmp(&other.cards[3].1))
                    .then(self.cards[4].1.cmp(&other.cards[4].1)),
                Category::ThreeOfAKind | Category::TwoPair => {
                    self_number_freq
                        .get(0)
                        .unwrap()
                        .0
                        .cmp(&other_number_freq.get(0).unwrap().0)
                        .then(
                            self_number_freq
                                .get(1)
                                .unwrap()
                                .0
                                .cmp(&other_number_freq.get(1).unwrap().0),
                        )
                        .then(
                            self_number_freq
                                .get(2)
                                .unwrap()
                                .0
                                .cmp(&other_number_freq.get(2).unwrap().0),
                        )
                }
                Category::OnePair => {
                    self_number_freq
                        .get(0)
                        .unwrap()
                        .0
                        .cmp(&other_number_freq.get(0).unwrap().0)
                        .then(
                            self_number_freq
                                .get(1)
                                .unwrap()
                                .0
                                .cmp(&other_number_freq.get(1).unwrap().0),
                        )
                        .then(
                            self_number_freq
                                .get(2)
                                .unwrap()
                                .0
                                .cmp(&other_number_freq.get(2).unwrap().0),
                        )
                        .then(
                            self_number_freq
                                .get(3)
                                .unwrap()
                                .0
                                .cmp(&other_number_freq.get(3).unwrap().0),
                        )                      
                }
                _ => Ordering::Equal,
            },
            ord => ord,
        };
        Some(ord)
    }
}

fn cards_number_freq(cards: &[Card]) -> Vec<(Number, usize)> {
    let mut cards_vec = cards
        .iter()
        .map(|c| (c.1, cards.iter().filter(|ca| ca.1 == c.1).count()))
        .collect::<Vec<_>>();
    cards_vec.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)).reverse());
    cards_vec
}

fn number_with_freq(cards: &Vec<(Number, usize)>, freq: usize) -> &(Number, usize) {
    cards.iter().filter(|n| n.1 == freq).next().unwrap()
}

fn cards_suit_freq(cards: &[Card]) -> Vec<(Suit, usize)> {
    cards
        .iter()
        .map(|c| (c.0, cards.iter().filter(|ca| ca.0 == c.0).count()))
        .collect()
}
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let parsed_hands = hands.iter().map(|&h| Hand::new(h)).collect::<Vec<_>>();
    unimplemented!("Out of {:?}, which hand wins?", hands)
}
