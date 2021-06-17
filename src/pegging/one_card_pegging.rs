use crate::card::Card;
use crate::pegging::Pegger;
use crate::CribbageCoreError;

#[derive(Clone, Copy)]
pub struct OneCardPegging {
    card: Card,
}

impl OneCardPegging {
    pub(in crate::pegging) fn new(card: Card) -> OneCardPegging {
        OneCardPegging { card }
    }

    pub fn play_card(
        self,
        card: Card,
        pegger: &mut Pegger,
    ) -> Result<u8, (Self, CribbageCoreError)> {
        if card != self.card {
            return Err((self, CribbageCoreError::InvalidCard));
        }

        match pegger.play_card(card) {
            Ok(points) => Ok(points),
            Err(error) => Err((self, error)),
        }
    }
}
