use std::error;
use std::fmt;

mod board;
mod card;
mod deck;
mod hand;
mod pegging;

pub use crate::board::{
    custom_board, standard_four_player_board, standard_three_player_board,
    standard_two_player_board, Board, FourPlayerScore, FourPlayers, ThreePlayerScore, ThreePlayers,
    TwoPlayerScore, TwoPlayers,
};
pub use crate::card::{Card, Rank, Suit};
pub use crate::deck::Deck;
pub use crate::hand::{
    deal_four_player_hand, deal_three_player_hand, deal_two_player_hand, CribCards,
    FourPlayerCribPart, FourPlayerDeal, Hand, KeptCards, ThreePlayerCribPart, ThreePlayerDeal,
    TwoPlayerCribPart, TwoPlayerDeal,
};
pub use crate::pegging::{
    FourCardPegging, OneCardPegging, Pegger, ThreeCardPegging, TwoCardPegging,
};

#[derive(Debug, Eq, PartialEq)]
pub enum CribbageCoreError {
    InvalidCard,
    InvalidCardString,
    InvalidScoreId,
    NotEnoughCards,
    WinnerExists,
}

impl fmt::Display for CribbageCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CribbageCoreError::InvalidCard
            | CribbageCoreError::InvalidCardString
            | CribbageCoreError::InvalidScoreId
            | CribbageCoreError::NotEnoughCards
            | CribbageCoreError::WinnerExists => write!(f, "{:?}", self),
        }
    }
}

impl error::Error for CribbageCoreError {
    fn description(&self) -> &str {
        match *self {
            CribbageCoreError::InvalidCard => "Invalid card played",
            CribbageCoreError::InvalidCardString => "Invalid string representation of card",
            CribbageCoreError::InvalidScoreId => "Invalid score ID",
            CribbageCoreError::NotEnoughCards => "Not enough cards in deck",
            CribbageCoreError::WinnerExists => "Winner already exists",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            CribbageCoreError::InvalidCard
            | CribbageCoreError::InvalidCardString
            | CribbageCoreError::InvalidScoreId
            | CribbageCoreError::NotEnoughCards
            | CribbageCoreError::WinnerExists => None,
        }
    }
}
