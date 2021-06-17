use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use crate::CribbageCoreError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
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
}

impl Rank {
    pub fn ordinal(self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }

    pub fn value(self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Suit::Hearts => "H",
            Suit::Clubs => "C",
            Suit::Diamonds => "D",
            Suit::Spades => "S",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn rank(self) -> Rank {
        self.rank
    }

    pub fn suit(self) -> Suit {
        self.suit
    }
}

impl FromStr for Card {
    type Err = CribbageCoreError;

    fn from_str(card_string: &str) -> Result<Card, CribbageCoreError> {
        let chars: Vec<char> = card_string.trim().chars().collect();
        if chars.len() != 2 {
            return Err(CribbageCoreError::InvalidCardString);
        }

        let rank = match chars[0].to_ascii_uppercase() {
            'A' => Rank::Ace,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            _ => return Err(CribbageCoreError::InvalidCardString),
        };

        let suit = match chars[1].to_ascii_uppercase() {
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'S' => Suit::Spades,
            _ => return Err(CribbageCoreError::InvalidCardString),
        };

        Ok(Card { rank, suit })
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod test {
    use crate::card::{Card, Rank, Suit};
    use crate::CribbageCoreError;
    use std::str::FromStr;

    const RANKS: [Rank; 13] = [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];

    const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades];

    #[test]
    fn test_rank_to_string() {
        assert_eq!(Rank::Ace.to_string(), "A".to_string());
        assert_eq!(Rank::Two.to_string(), "2".to_string());
        assert_eq!(Rank::Three.to_string(), "3".to_string());
        assert_eq!(Rank::Four.to_string(), "4".to_string());
        assert_eq!(Rank::Five.to_string(), "5".to_string());
        assert_eq!(Rank::Six.to_string(), "6".to_string());
        assert_eq!(Rank::Seven.to_string(), "7".to_string());
        assert_eq!(Rank::Eight.to_string(), "8".to_string());
        assert_eq!(Rank::Nine.to_string(), "9".to_string());
        assert_eq!(Rank::Ten.to_string(), "T".to_string());
        assert_eq!(Rank::Jack.to_string(), "J".to_string());
        assert_eq!(Rank::Queen.to_string(), "Q".to_string());
        assert_eq!(Rank::King.to_string(), "K".to_string());
    }

    #[test]
    fn test_suit_to_string() {
        assert_eq!(Suit::Hearts.to_string(), "H".to_string());
        assert_eq!(Suit::Clubs.to_string(), "C".to_string());
        assert_eq!(Suit::Diamonds.to_string(), "D".to_string());
        assert_eq!(Suit::Spades.to_string(), "S".to_string());
    }

    #[test]
    fn test_card_to_string() {
        for rank in &RANKS {
            for suit in &SUITS {
                let expected = format!("{}{}", rank.to_string(), suit.to_string());
                let card = Card {
                    rank: *rank,
                    suit: *suit,
                };
                assert_eq!(card.to_string(), expected);
            }
        }
    }

    #[test]
    fn test_card_from_str() {
        for rank in &RANKS {
            for suit in &SUITS {
                let card_string = format!("{}{}", rank.to_string(), suit.to_string());
                let expected = Card {
                    rank: *rank,
                    suit: *suit,
                };
                let card = Card::from_str(&card_string).unwrap();
                assert_eq!(card, expected);
            }
        }

        assert_eq!(
            Card::from_str(" AS"),
            Ok(Card {
                rank: Rank::Ace,
                suit: Suit::Spades
            })
        );
        assert_eq!(
            Card::from_str("AS "),
            Ok(Card {
                rank: Rank::Ace,
                suit: Suit::Spades
            })
        );

        assert_eq!(
            Card::from_str(""),
            Err(CribbageCoreError::InvalidCardString)
        );
        assert_eq!(
            Card::from_str("ASA"),
            Err(CribbageCoreError::InvalidCardString)
        );
        assert_eq!(
            Card::from_str("AX"),
            Err(CribbageCoreError::InvalidCardString)
        );
        assert_eq!(
            Card::from_str("XS"),
            Err(CribbageCoreError::InvalidCardString)
        );
    }
}
