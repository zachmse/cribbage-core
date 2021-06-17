use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Rank, Suit};
use crate::CribbageCoreError;

pub struct Deck {
    cards_drawn: usize,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck::default()
    }

    pub fn draw(&mut self) -> Result<Card, CribbageCoreError> {
        let card = match self.cards.get(self.cards_drawn) {
            Some(card) => {
                self.cards_drawn += 1;
                card
            }
            None => return Err(CribbageCoreError::NotEnoughCards),
        };

        Ok(*card)
    }

    pub fn draw_n(&mut self, n: usize) -> Result<Vec<Card>, CribbageCoreError> {
        let start = self.cards_drawn;
        let end = self.cards_drawn + n;
        if end > self.cards.len() {
            return Err(CribbageCoreError::NotEnoughCards);
        }

        self.cards_drawn = end;
        Ok(self.cards[start..end].to_vec())
    }

    pub fn shuffle(&mut self) {
        self.cards_drawn = 0;
        self.cards.shuffle(&mut thread_rng());
    }
}

impl Default for Deck {
    fn default() -> Deck {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
        ];

        Deck {
            cards_drawn: 0,
            cards,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Rank, Suit};
    use crate::deck::Deck;
    use crate::CribbageCoreError;

    #[test]
    fn test_new() {
        let deck = Deck::new();
        assert_eq!(deck.cards_drawn, 0);
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_new_shuffle() {
        let mut deck = Deck::new();
        deck.shuffle();
        assert_ne!(deck.cards, Deck::new().cards);
    }

    #[test]
    pub fn test_draw() {
        let mut deck = Deck::new();
        assert_eq!(deck.cards_drawn, 0);

        assert_eq!(deck.draw().unwrap(), Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(deck.cards_drawn, 1);

        for _ in 0..51 {
            deck.draw().unwrap();
        }

        assert_eq!(deck.cards_drawn, 52);

        assert_eq!(deck.draw(), Err(CribbageCoreError::NotEnoughCards));
        assert_eq!(deck.cards_drawn, 52);
    }

    #[test]
    pub fn test_draw_n() {
        let mut deck = Deck::new();
        assert_eq!(deck.draw_n(0).unwrap(), Vec::new());
        assert_eq!(deck.cards_drawn, 0);

        assert_eq!(
            deck.draw_n(1).unwrap(),
            vec![Card::new(Rank::Ace, Suit::Hearts)]
        );
        assert_eq!(deck.cards_drawn, 1);

        deck = Deck::new();
        assert_eq!(deck.draw_n(52).unwrap(), Deck::new().cards);
        assert_eq!(deck.cards_drawn, 52);

        assert_eq!(deck.draw_n(0).unwrap(), Vec::new());
        assert_eq!(deck.cards_drawn, 52);

        assert_eq!(deck.draw_n(1), Err(CribbageCoreError::NotEnoughCards));
        assert_eq!(deck.cards_drawn, 52);

        assert_eq!(deck.draw_n(52), Err(CribbageCoreError::NotEnoughCards));
        assert_eq!(deck.cards_drawn, 52);
    }

    #[test]
    pub fn test_shuffle() {
        let mut deck = Deck::new();
        deck.shuffle();
        assert_ne!(deck.cards, Deck::new().cards);

        deck.draw().unwrap();
        assert_eq!(deck.cards_drawn, 1);

        deck.shuffle();
        assert_eq!(deck.cards_drawn, 0);

        deck.draw_n(52).unwrap();
        assert_eq!(deck.cards_drawn, 52);

        deck.shuffle();
        assert_eq!(deck.cards_drawn, 0);
    }
}
