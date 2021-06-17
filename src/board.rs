use std::ops::{Index, IndexMut};

use crate::CribbageCoreError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TwoPlayers {
    PlayerOne,
    PlayerTwo,
}

pub struct TwoPlayerScore<T> {
    scores: [T; 2],
}

impl<T> Index<TwoPlayers> for TwoPlayerScore<T> {
    type Output = T;

    fn index(&self, index: TwoPlayers) -> &Self::Output {
        match index {
            TwoPlayers::PlayerOne => &self.scores[0],
            TwoPlayers::PlayerTwo => &self.scores[1],
        }
    }
}

impl<T> IndexMut<TwoPlayers> for TwoPlayerScore<T> {
    fn index_mut(&mut self, index: TwoPlayers) -> &mut T {
        match index {
            TwoPlayers::PlayerOne => &mut self.scores[0],
            TwoPlayers::PlayerTwo => &mut self.scores[1],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreePlayers {
    PlayerOne,
    PlayerTwo,
    PlayerThree,
}

pub struct ThreePlayerScore<T> {
    scores: [T; 3],
}

impl<T> Index<ThreePlayers> for ThreePlayerScore<T> {
    type Output = T;

    fn index(&self, index: ThreePlayers) -> &Self::Output {
        match index {
            ThreePlayers::PlayerOne => &self.scores[0],
            ThreePlayers::PlayerTwo => &self.scores[1],
            ThreePlayers::PlayerThree => &self.scores[2],
        }
    }
}

impl<T> IndexMut<ThreePlayers> for ThreePlayerScore<T> {
    fn index_mut(&mut self, index: ThreePlayers) -> &mut T {
        match index {
            ThreePlayers::PlayerOne => &mut self.scores[0],
            ThreePlayers::PlayerTwo => &mut self.scores[1],
            ThreePlayers::PlayerThree => &mut self.scores[2],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FourPlayers {
    PlayerOne,
    PlayerTwo,
    PlayerThree,
    PlayerFour,
}

pub struct FourPlayerScore<T> {
    scores: [T; 4],
}

impl<T> Index<FourPlayers> for FourPlayerScore<T> {
    type Output = T;

    fn index(&self, index: FourPlayers) -> &Self::Output {
        match index {
            FourPlayers::PlayerOne => &self.scores[0],
            FourPlayers::PlayerTwo => &self.scores[1],
            FourPlayers::PlayerThree => &self.scores[2],
            FourPlayers::PlayerFour => &self.scores[3],
        }
    }
}

impl<T> IndexMut<FourPlayers> for FourPlayerScore<T> {
    fn index_mut(&mut self, index: FourPlayers) -> &mut T {
        match index {
            FourPlayers::PlayerOne => &mut self.scores[0],
            FourPlayers::PlayerTwo => &mut self.scores[1],
            FourPlayers::PlayerThree => &mut self.scores[2],
            FourPlayers::PlayerFour => &mut self.scores[3],
        }
    }
}

pub trait Score: Copy + PartialEq + PartialOrd + Sized {
    fn initial() -> Self;

    fn saturating_add(self, other: Self) -> Self;

    fn saturating_sub(self, other: Self) -> Self;
}

impl Score for u8 {
    fn initial() -> Self {
        0
    }

    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }

    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}

pub struct Board<P, S>
where
    S: Index<P>,
    S::Output: Sized,
{
    scores: S,
    target: S::Output,
    winner: Option<P>,
}

impl<P, S> Board<P, S>
where
    P: Copy,
    S: Index<P> + IndexMut<P>,
    S::Output: Score,
{
    pub fn add_points(&mut self, id: P, points: S::Output) -> Result<S::Output, CribbageCoreError> {
        if self.winner.is_some() {
            return Err(CribbageCoreError::WinnerExists);
        }

        self.scores[id] = self.scores[id].saturating_add(points);
        if self.scores[id] > self.target {
            self.scores[id] = self.target;
        }

        if self.scores[id] == self.target {
            self.winner = Some(id);
        }

        Ok(self.scores[id])
    }

    pub fn subtract_points(
        &mut self,
        id: P,
        points: S::Output,
    ) -> Result<S::Output, CribbageCoreError> {
        if self.winner.is_some() {
            return Err(CribbageCoreError::WinnerExists);
        }

        self.scores[id] = self.scores[id].saturating_sub(points);
        Ok(self.scores[id])
    }

    pub fn score(&self, id: P) -> S::Output {
        self.scores[id]
    }

    pub fn winner(&self) -> Option<P> {
        self.winner
    }
}

pub fn custom_board<P, S>(scores: S, target: S::Output) -> Board<P, S>
where
    S: Index<P>,
    S::Output: Sized,
{
    Board {
        scores,
        target,
        winner: None,
    }
}

pub fn standard_two_player_board() -> Board<TwoPlayers, TwoPlayerScore<u8>> {
    custom_board(TwoPlayerScore { scores: [0; 2] }, 121)
}

pub fn standard_three_player_board() -> Board<ThreePlayers, ThreePlayerScore<u8>> {
    custom_board(ThreePlayerScore { scores: [0; 3] }, 121)
}

pub fn standard_four_player_board() -> Board<FourPlayers, FourPlayerScore<u8>> {
    custom_board(FourPlayerScore { scores: [0; 4] }, 121)
}

#[cfg(test)]
mod tests {
    use crate::board::{
        standard_four_player_board, standard_three_player_board, standard_two_player_board,
        FourPlayers, ThreePlayers, TwoPlayers,
    };
    use crate::CribbageCoreError;

    #[test]
    pub fn test_two_player_board() {
        let mut board = standard_two_player_board();
        assert_eq!(board.score(TwoPlayers::PlayerOne), 0);
        assert_eq!(board.score(TwoPlayers::PlayerTwo), 0);

        assert!(board.add_points(TwoPlayers::PlayerOne, 1).is_ok());
        assert_eq!(board.score(TwoPlayers::PlayerOne), 1);
        assert_eq!(board.score(TwoPlayers::PlayerTwo), 0);

        assert!(board.add_points(TwoPlayers::PlayerOne, 119).is_ok());
        assert_eq!(board.score(TwoPlayers::PlayerOne), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(TwoPlayers::PlayerOne, 2).is_ok());
        assert_eq!(board.score(TwoPlayers::PlayerOne), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(TwoPlayers::PlayerOne));

        assert_eq!(
            board.add_points(TwoPlayers::PlayerOne, 1),
            Err(CribbageCoreError::WinnerExists)
        );

        let mut board = standard_two_player_board();
        assert!(board.add_points(TwoPlayers::PlayerTwo, 1).is_ok());
        assert_eq!(board.score(TwoPlayers::PlayerOne), 0);
        assert_eq!(board.score(TwoPlayers::PlayerTwo), 1);

        assert!(board.add_points(TwoPlayers::PlayerTwo, 119).is_ok());
        assert_eq!(board.score(TwoPlayers::PlayerTwo), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(TwoPlayers::PlayerTwo, 2).is_ok());
        assert_eq!(board.score(TwoPlayers::PlayerTwo), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(TwoPlayers::PlayerTwo));

        assert_eq!(
            board.add_points(TwoPlayers::PlayerTwo, 1),
            Err(CribbageCoreError::WinnerExists)
        );
    }

    #[test]
    pub fn test_three_player_board() {
        let mut board = standard_three_player_board();
        assert_eq!(board.score(ThreePlayers::PlayerOne), 0);
        assert_eq!(board.score(ThreePlayers::PlayerTwo), 0);
        assert_eq!(board.score(ThreePlayers::PlayerThree), 0);

        assert!(board.add_points(ThreePlayers::PlayerOne, 1).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerOne), 1);
        assert_eq!(board.score(ThreePlayers::PlayerTwo), 0);
        assert_eq!(board.score(ThreePlayers::PlayerThree), 0);

        assert!(board.add_points(ThreePlayers::PlayerOne, 119).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerOne), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(ThreePlayers::PlayerOne, 2).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerOne), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(ThreePlayers::PlayerOne));

        assert_eq!(
            board.add_points(ThreePlayers::PlayerOne, 1),
            Err(CribbageCoreError::WinnerExists)
        );

        let mut board = standard_three_player_board();
        assert!(board.add_points(ThreePlayers::PlayerTwo, 1).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerOne), 0);
        assert_eq!(board.score(ThreePlayers::PlayerTwo), 1);
        assert_eq!(board.score(ThreePlayers::PlayerThree), 0);

        assert!(board.add_points(ThreePlayers::PlayerTwo, 119).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerTwo), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(ThreePlayers::PlayerTwo, 2).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerTwo), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(ThreePlayers::PlayerTwo));

        assert_eq!(
            board.add_points(ThreePlayers::PlayerTwo, 1),
            Err(CribbageCoreError::WinnerExists)
        );

        let mut board = standard_three_player_board();
        assert!(board.add_points(ThreePlayers::PlayerThree, 1).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerOne), 0);
        assert_eq!(board.score(ThreePlayers::PlayerTwo), 0);
        assert_eq!(board.score(ThreePlayers::PlayerThree), 1);

        assert!(board.add_points(ThreePlayers::PlayerThree, 119).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerThree), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(ThreePlayers::PlayerThree, 2).is_ok());
        assert_eq!(board.score(ThreePlayers::PlayerThree), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(ThreePlayers::PlayerThree));

        assert_eq!(
            board.add_points(ThreePlayers::PlayerThree, 1),
            Err(CribbageCoreError::WinnerExists)
        );
    }

    #[test]
    pub fn test_four_player_board() {
        let mut board = standard_four_player_board();
        assert_eq!(board.score(FourPlayers::PlayerOne), 0);
        assert_eq!(board.score(FourPlayers::PlayerTwo), 0);
        assert_eq!(board.score(FourPlayers::PlayerThree), 0);
        assert_eq!(board.score(FourPlayers::PlayerFour), 0);

        assert!(board.add_points(FourPlayers::PlayerOne, 1).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerOne), 1);
        assert_eq!(board.score(FourPlayers::PlayerTwo), 0);
        assert_eq!(board.score(FourPlayers::PlayerThree), 0);
        assert_eq!(board.score(FourPlayers::PlayerFour), 0);

        assert!(board.add_points(FourPlayers::PlayerOne, 119).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerOne), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(FourPlayers::PlayerOne, 2).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerOne), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(FourPlayers::PlayerOne));

        assert_eq!(
            board.add_points(FourPlayers::PlayerOne, 1),
            Err(CribbageCoreError::WinnerExists)
        );

        let mut board = standard_four_player_board();
        assert!(board.add_points(FourPlayers::PlayerTwo, 1).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerOne), 0);
        assert_eq!(board.score(FourPlayers::PlayerTwo), 1);
        assert_eq!(board.score(FourPlayers::PlayerThree), 0);
        assert_eq!(board.score(FourPlayers::PlayerFour), 0);

        assert!(board.add_points(FourPlayers::PlayerTwo, 119).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerTwo), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(FourPlayers::PlayerTwo, 2).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerTwo), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(FourPlayers::PlayerTwo));

        assert_eq!(
            board.add_points(FourPlayers::PlayerTwo, 1),
            Err(CribbageCoreError::WinnerExists)
        );

        let mut board = standard_four_player_board();
        assert!(board.add_points(FourPlayers::PlayerThree, 1).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerOne), 0);
        assert_eq!(board.score(FourPlayers::PlayerTwo), 0);
        assert_eq!(board.score(FourPlayers::PlayerThree), 1);
        assert_eq!(board.score(FourPlayers::PlayerFour), 0);

        assert!(board.add_points(FourPlayers::PlayerThree, 119).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerThree), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(FourPlayers::PlayerThree, 2).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerThree), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(FourPlayers::PlayerThree));

        assert_eq!(
            board.add_points(FourPlayers::PlayerThree, 1),
            Err(CribbageCoreError::WinnerExists)
        );

        let mut board = standard_four_player_board();
        assert!(board.add_points(FourPlayers::PlayerFour, 1).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerOne), 0);
        assert_eq!(board.score(FourPlayers::PlayerTwo), 0);
        assert_eq!(board.score(FourPlayers::PlayerThree), 0);
        assert_eq!(board.score(FourPlayers::PlayerFour), 1);

        assert!(board.add_points(FourPlayers::PlayerFour, 119).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerFour), 120);
        assert_eq!(board.winner().is_some(), false);
        assert_eq!(board.winner(), None);

        assert!(board.add_points(FourPlayers::PlayerFour, 2).is_ok());
        assert_eq!(board.score(FourPlayers::PlayerFour), 121);
        assert_eq!(board.winner().is_some(), true);
        assert_eq!(board.winner(), Some(FourPlayers::PlayerFour));

        assert_eq!(
            board.add_points(FourPlayers::PlayerFour, 1),
            Err(CribbageCoreError::WinnerExists)
        );
    }
}
