use crate::card::Card;
use crate::hand::{FourPlayerCribPart, KeptCards, ThreePlayerCribPart, TwoPlayerCribPart};

pub struct TwoPlayerDeal {
    cards: [Card; 6],
}

impl TwoPlayerDeal {
    pub fn new(cards: [Card; 6]) -> TwoPlayerDeal {
        TwoPlayerDeal { cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn split(self, keep: [Card; 4], crib: [Card; 2]) -> (KeptCards, TwoPlayerCribPart) {
        // Todo: Make sure cards specified exist in self.cards
        (KeptCards::new(keep), TwoPlayerCribPart::new(crib))
    }
}

pub struct ThreePlayerDeal {
    cards: [Card; 5],
}

impl ThreePlayerDeal {
    pub fn new(cards: [Card; 5]) -> ThreePlayerDeal {
        ThreePlayerDeal { cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn split(self, keep: [Card; 4], crib: Card) -> (KeptCards, ThreePlayerCribPart) {
        (KeptCards::new(keep), ThreePlayerCribPart::new(crib))
    }
}

pub struct FourPlayerDeal {
    cards: [Card; 5],
}

impl FourPlayerDeal {
    pub fn new(cards: [Card; 5]) -> FourPlayerDeal {
        FourPlayerDeal { cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn split(self, keep: [Card; 4], crib: Card) -> (KeptCards, FourPlayerCribPart) {
        (KeptCards::new(keep), FourPlayerCribPart::new(crib))
    }
}
