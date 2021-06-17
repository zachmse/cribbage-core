use crate::card::Card;
use crate::pegging::{OneCardPegging, Pegger};
use crate::CribbageCoreError;

#[derive(Clone, Copy)]
pub struct TwoCardPegging {
    cards: [Card; 2],
}

impl TwoCardPegging {
    pub(in crate::pegging) fn new(cards: [Card; 2]) -> TwoCardPegging {
        TwoCardPegging { cards }
    }

    pub fn play_card(
        self,
        card: Card,
        pegger: &mut Pegger,
    ) -> Result<(u8, OneCardPegging), (Self, CribbageCoreError)> {
        let mut cards = self.cards[..].to_vec();
        let index = cards[..]
            .iter()
            .position(|&c| c == card)
            .ok_or((self, CribbageCoreError::InvalidCard))?;
        cards.swap_remove(index);

        match pegger.play_card(card) {
            Ok(points) => Ok((points, OneCardPegging::new(cards[0]))),
            Err(error) => Err((self, error)),
        }
    }
}
