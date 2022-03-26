use std::vec;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

enum Rank {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

struct Hand<'a> {
    hand: &'a str,
    rank: Rank,
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(hand: &'a str) -> Self {
        Self {
            hand,
            rank: Rank::HighCard,
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands: Vec<Hand> = hands.iter().map(|&h| h.into()).collect();
    vec![]
}
