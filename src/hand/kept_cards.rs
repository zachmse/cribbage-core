use crate::card::Card;
use crate::hand::Hand;

pub struct KeptCards {
    cards: [Card; 4],
}

impl KeptCards {
    pub(in crate::hand) fn new(cards: [Card; 4]) -> KeptCards {
        KeptCards { cards }
    }

    pub fn add_cut_card(self, cut: Card) -> Hand {
        Hand::new(self.cards, cut, false)
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}
