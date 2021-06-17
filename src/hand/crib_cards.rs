use crate::card::Card;
use crate::hand::Hand;

pub struct CribCards {
    cards: [Card; 4],
}

impl CribCards {
    pub(in crate::hand) fn new(cards: [Card; 4]) -> CribCards {
        CribCards { cards }
    }

    pub fn add_cut_card(self, cut: Card) -> Hand {
        Hand::new(self.cards, cut, true)
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}
