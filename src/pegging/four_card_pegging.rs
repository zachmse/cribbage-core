use crate::card::Card;
use crate::pegging::{Pegger, ThreeCardPegging};
use crate::CribbageCoreError;

#[derive(Clone, Copy)]
pub struct FourCardPegging {
    cards: [Card; 4],
}

impl FourCardPegging {
    pub fn new(cards: [Card; 4]) -> FourCardPegging {
        FourCardPegging { cards }
    }

    pub fn play_card(
        self,
        card: Card,
        pegger: &mut Pegger,
    ) -> Result<(u8, ThreeCardPegging), (Self, CribbageCoreError)> {
        let mut cards = self.cards[..].to_vec();
        let index = cards[..]
            .iter()
            .position(|&c| c == card)
            .ok_or((self, CribbageCoreError::InvalidCard))?;
        cards.swap_remove(index);

        match pegger.play_card(card) {
            Ok(points) => Ok((
                points,
                ThreeCardPegging::new([cards[0], cards[1], cards[2]]),
            )),
            Err(error) => Err((self, error)),
        }
    }
}
