mod four_card_pegging;
mod one_card_pegging;
mod three_card_pegging;
mod two_card_pegging;

pub use self::four_card_pegging::FourCardPegging;
pub use self::one_card_pegging::OneCardPegging;
pub use self::three_card_pegging::ThreeCardPegging;
pub use self::two_card_pegging::TwoCardPegging;

use crate::card::Card;
use crate::CribbageCoreError;

#[derive(Default)]
pub struct Pegger {
    count: u8,
    played_cards: Vec<Card>,
}

impl Pegger {
    pub fn new() -> Pegger {
        Pegger {
            count: 0,
            played_cards: Vec::new(),
        }
    }

    fn is_run(cards: &[Card]) -> bool {
        if cards.len() < 3 {
            return false;
        }

        let mut sorted = cards.to_vec();
        sorted.sort();

        for i in 1..sorted.len() {
            if sorted[i].rank().ordinal() != sorted[i - 1].rank().ordinal() + 1 {
                return false;
            }
        }

        true
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    /*
    Note: Caller must implement logic to assign the "go" point to the appropriate player
          whenever a count of 31 isn't reached exactly. In the event that a count of 31 is reached
          exactly, this method returns the 1 point (expecting the caller to add 1 point for
          the "go").
    */
    pub fn play_card(&mut self, card: Card) -> Result<u8, CribbageCoreError> {
        let new_count = self.count + card.rank().value();
        if new_count > 31 {
            return Err(CribbageCoreError::InvalidCard);
        }

        self.count = new_count;

        let mut points = match self.count {
            15 => 2,
            31 => 1,
            _ => 0,
        };

        let mut cards_with_same_rank = 0;
        for played_card in self.played_cards.iter().rev() {
            if played_card.rank() == card.rank() {
                cards_with_same_rank += 1;
            } else {
                break;
            }
        }

        self.played_cards.push(card);

        points += match cards_with_same_rank {
            1 => 2,
            2 => 6,
            3 => 12,
            _ => 0,
        };

        for i in 0..self.played_cards.len() {
            let card_slice = &self.played_cards[i..self.played_cards.len()];
            if Pegger::is_run(card_slice) {
                points += card_slice.len() as u8;
                break;
            }
        }

        Ok(points)
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.played_cards.clear();
    }
}

#[cfg(test)]
mod test {
    use crate::card::{Card, Rank, Suit};
    use crate::CribbageCoreError;
    use crate::Pegger;

    #[test]
    fn test_is_run() {
        assert_eq!(Pegger::is_run(&[]), false);
        assert_eq!(Pegger::is_run(&[Card::new(Rank::Ace, Suit::Spades)]), false);
        assert_eq!(
            Pegger::is_run(&[
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades)
            ]),
            false
        );
        assert_eq!(
            Pegger::is_run(&[
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Three, Suit::Spades)
            ]),
            true
        );
        assert_eq!(
            Pegger::is_run(&[
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Four, Suit::Spades)
            ]),
            false
        );
        assert_eq!(
            Pegger::is_run(&[
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Ace, Suit::Spades)
            ]),
            true
        );
        assert_eq!(
            Pegger::is_run(&[
                Card::new(Rank::Ace, Suit::Hearts),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Three, Suit::Diamonds),
                Card::new(Rank::Four, Suit::Spades)
            ]),
            true
        );
        assert_eq!(
            Pegger::is_run(&[
                Card::new(Rank::Four, Suit::Hearts),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Diamonds),
                Card::new(Rank::Three, Suit::Spades)
            ]),
            true
        );
        assert_eq!(
            Pegger::is_run(&[
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Five, Suit::Spades)
            ]),
            false
        );
    }

    #[test]
    fn test_play_card() {
        let mut pegger = Pegger::new();
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Ace, Suit::Spades))
                .unwrap(),
            0
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Ace, Suit::Hearts))
                .unwrap(),
            2
        );
        assert_eq!(
            pegger.play_card(Card::new(Rank::Ace, Suit::Clubs)).unwrap(),
            6
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Ace, Suit::Diamonds))
                .unwrap(),
            12
        );
        assert_eq!(
            pegger.play_card(Card::new(Rank::Two, Suit::Clubs)).unwrap(),
            0
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Three, Suit::Spades))
                .unwrap(),
            3
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Four, Suit::Diamonds))
                .unwrap(),
            4
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Two, Suit::Hearts))
                .unwrap(),
            5
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Eight, Suit::Clubs))
                .unwrap(),
            0
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Eight, Suit::Spades))
                .unwrap(),
            3
        );
        pegger.reset();

        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Ten, Suit::Spades))
                .unwrap(),
            0
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Five, Suit::Spades))
                .unwrap(),
            2
        );
        assert_eq!(
            pegger.play_card(Card::new(Rank::Ten, Suit::Clubs)).unwrap(),
            0
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Five, Suit::Diamonds))
                .unwrap(),
            0
        );
        assert_eq!(
            pegger
                .play_card(Card::new(Rank::Ace, Suit::Hearts))
                .unwrap(),
            1
        );
        assert_eq!(
            pegger.play_card(Card::new(Rank::Ace, Suit::Spades)),
            Err(CribbageCoreError::InvalidCard)
        );
    }
}
