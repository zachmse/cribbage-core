use crate::card::Card;
use crate::hand::CribCards;

pub struct TwoPlayerCribPart {
    cards: [Card; 2],
}

impl TwoPlayerCribPart {
    pub(in crate::hand) fn new(cards: [Card; 2]) -> TwoPlayerCribPart {
        TwoPlayerCribPart { cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
    pub fn combine(self, other: TwoPlayerCribPart) -> CribCards {
        CribCards::new([self.cards[0], self.cards[1], other.cards[0], other.cards[1]])
    }
}

pub struct ThreePlayerCribPart {
    card: Card,
}

impl ThreePlayerCribPart {
    pub(in crate::hand) fn new(card: Card) -> ThreePlayerCribPart {
        ThreePlayerCribPart { card }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
    pub fn combine(
        self,
        other1: ThreePlayerCribPart,
        other2: ThreePlayerCribPart,
        starter: Card,
    ) -> CribCards {
        CribCards::new([self.card, other1.card, other2.card, starter])
    }
}

pub struct FourPlayerCribPart {
    card: Card,
}

impl FourPlayerCribPart {
    pub(in crate::hand) fn new(card: Card) -> FourPlayerCribPart {
        FourPlayerCribPart { card }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_pass_by_value))]
    pub fn combine(
        self,
        other1: FourPlayerCribPart,
        other2: FourPlayerCribPart,
        other3: FourPlayerCribPart,
    ) -> CribCards {
        CribCards::new([self.card, other1.card, other2.card, other3.card])
    }
}
