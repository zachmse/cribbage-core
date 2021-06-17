use crate::card::Card;
use crate::pegging::{Pegger, TwoCardPegging};
use crate::CribbageCoreError;

#[derive(Clone, Copy)]
pub struct ThreeCardPegging {
    cards: [Card; 3],
}

impl ThreeCardPegging {
    pub(in crate::pegging) fn new(cards: [Card; 3]) -> ThreeCardPegging {
        ThreeCardPegging { cards }
    }

    pub fn play_card(
        self,
        card: Card,
        pegger: &mut Pegger,
    ) -> Result<(u8, TwoCardPegging), (Self, CribbageCoreError)> {
        let mut cards = self.cards[..].to_vec();
        let index = cards[..]
            .iter()
            .position(|&c| c == card)
            .ok_or((self, CribbageCoreError::InvalidCard))?;
        cards.swap_remove(index);

        match pegger.play_card(card) {
            Ok(points) => Ok((points, TwoCardPegging::new([cards[0], cards[1]]))),
            Err(error) => Err((self, error)),
        }
    }
}
