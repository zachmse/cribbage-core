mod crib_cards;
mod crib_part;
mod dealt_cards;
mod kept_cards;

use crate::card::{Card, Rank};
use crate::deck::Deck;
use crate::CribbageCoreError;

pub use self::crib_cards::CribCards;
pub use self::crib_part::FourPlayerCribPart;
pub use self::crib_part::ThreePlayerCribPart;
pub use self::crib_part::TwoPlayerCribPart;
pub use self::dealt_cards::FourPlayerDeal;
pub use self::dealt_cards::ThreePlayerDeal;
pub use self::dealt_cards::TwoPlayerDeal;
pub use self::kept_cards::KeptCards;

pub fn deal_four_player_hand(deck: &mut Deck) -> Result<FourPlayerDeal, CribbageCoreError> {
    let cards = deck.draw_n(5)?;
    Ok(FourPlayerDeal::new([
        cards[0], cards[1], cards[2], cards[3], cards[4],
    ]))
}

pub fn deal_three_player_hand(deck: &mut Deck) -> Result<ThreePlayerDeal, CribbageCoreError> {
    let cards = deck.draw_n(5)?;
    Ok(ThreePlayerDeal::new([
        cards[0], cards[1], cards[2], cards[3], cards[4],
    ]))
}

pub fn deal_two_player_hand(deck: &mut Deck) -> Result<TwoPlayerDeal, CribbageCoreError> {
    let cards = deck.draw_n(6)?;
    Ok(TwoPlayerDeal::new([
        cards[0], cards[1], cards[2], cards[3], cards[4], cards[5],
    ]))
}

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 4],
    cut: Card,
    is_crib: bool,
    score: Option<u8>,
}

impl Hand {
    pub fn new(cards: [Card; 4], cut: Card, is_crib: bool) -> Hand {
        Hand {
            cards,
            cut,
            is_crib,
            score: None,
        }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn score(&mut self) -> u8 {
        if let Some(score) = self.score {
            return score;
        }

        let mut score = 0u8;
        score += self.score_fifteens();
        score += self.score_pairs();
        score += self.score_runs();
        score += self.score_flush();
        score += self.score_nobs();
        self.score = Some(score);
        score
    }

    fn score_fifteens(&self) -> u8 {
        let mut points = 0u8;
        let subsets = vec![
            self.sets_of_two(),
            self.sets_of_three(),
            self.sets_of_four(),
            self.sets_of_five(),
        ];

        for subset in subsets {
            for set in subset {
                let mut sum = 0u8;
                for card in &set {
                    sum += card.rank().value();
                }

                if sum == 15 {
                    points += 2;
                }
            }
        }

        points
    }

    fn score_flush(&self) -> u8 {
        let mut points = 0;
        let suit = self.cards[0].suit();
        if self.cards[1..4].iter().all(|c| c.suit() == suit) {
            points += 4;

            if self.cut.suit() == suit {
                points += 1;
            } else if self.is_crib {
                points = 0;
            }
        }

        points
    }

    fn score_nobs(&self) -> u8 {
        for card in &self.cards {
            if card.rank() == Rank::Jack && card.suit() == self.cut.suit() {
                return 1;
            }
        }

        0
    }

    fn score_pairs(&self) -> u8 {
        let mut points = 0u8;
        let sets_of_two = self.sets_of_two();
        for set in sets_of_two {
            if set[0].rank() == set[1].rank() {
                points += 2u8;
            }
        }

        points
    }

    fn score_runs(&self) -> u8 {
        let mut points = 0u8;
        let mut temp = 0u8;

        for mut set in self.sets_of_five() {
            set.sort();
            temp += Hand::score_run(&set);
        }

        points += temp;
        if points != 0u8 {
            return points;
        }

        for mut set in self.sets_of_four() {
            set.sort();
            temp += Hand::score_run(&set);
        }

        points += temp;
        if points != 0u8 {
            return points;
        }

        for mut set in self.sets_of_three() {
            set.sort();
            temp += Hand::score_run(&set);
        }

        points += temp;
        if points != 0u8 {
            return points;
        }

        points
    }

    fn score_run(cards: &[Card]) -> u8 {
        if cards.len() < 3usize {
            return 0u8;
        }

        let mut points = 1u8;
        for i in 1..cards.len() {
            if cards[i].rank().ordinal() == (cards[i - 1].rank().ordinal() + 1) {
                points += 1u8;
            } else {
                points = 0u8;
                break;
            }
        }

        points
    }

    fn sets_of_two(&self) -> Vec<Vec<Card>> {
        vec![
            vec![self.cards[0], self.cards[1]],
            vec![self.cards[0], self.cards[2]],
            vec![self.cards[0], self.cards[3]],
            vec![self.cards[0], self.cut],
            vec![self.cards[1], self.cards[2]],
            vec![self.cards[1], self.cards[3]],
            vec![self.cards[1], self.cut],
            vec![self.cards[2], self.cards[3]],
            vec![self.cards[2], self.cut],
            vec![self.cards[3], self.cut],
        ]
    }

    fn sets_of_three(&self) -> Vec<Vec<Card>> {
        vec![
            vec![self.cards[0], self.cards[1], self.cards[2]],
            vec![self.cards[0], self.cards[1], self.cards[3]],
            vec![self.cards[0], self.cards[2], self.cards[3]],
            vec![self.cards[1], self.cards[2], self.cards[3]],
            vec![self.cards[0], self.cards[1], self.cut],
            vec![self.cards[0], self.cards[2], self.cut],
            vec![self.cards[1], self.cards[2], self.cut],
            vec![self.cards[0], self.cards[3], self.cut],
            vec![self.cards[1], self.cards[3], self.cut],
            vec![self.cards[2], self.cards[3], self.cut],
        ]
    }

    fn sets_of_four(&self) -> Vec<Vec<Card>> {
        vec![
            vec![self.cards[0], self.cards[1], self.cards[2], self.cards[3]],
            vec![self.cards[0], self.cards[1], self.cards[2], self.cut],
            vec![self.cards[0], self.cards[1], self.cards[3], self.cut],
            vec![self.cards[0], self.cards[2], self.cards[3], self.cut],
            vec![self.cards[1], self.cards[2], self.cards[3], self.cut],
        ]
    }

    fn sets_of_five(&self) -> Vec<Vec<Card>> {
        vec![vec![
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cut,
        ]]
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::hand::Hand;
    use std::str::FromStr;

    #[allow(dead_code)]
    #[cfg_attr(feature = "extensive-tests", test)]
    fn test_hands() {
        use std::env;
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use std::path::PathBuf;

        let key = "CRIBBAGE_CORE_RESOURCES_DIRECTORY";
        let resources = match env::var(key) {
            Ok(val) => val,
            Err(_) => panic!("Environment variable {} not set", key),
        };

        let mut path = PathBuf::new();
        path.push(resources);
        path.push("cribbage_hand_scores");

        let filenames = [
            "A.txt", "2.txt", "3.txt", "4.txt", "5.txt", "6.txt", "7.txt", "8.txt", "9.txt",
            "T.txt", "J.txt", "Q.txt", "K.txt",
        ];

        for filename in &filenames {
            path.push(filename);
            println!("Processing: {:?}", path);
            let reader = BufReader::new(File::open(&path).unwrap());
            for (line_number, line) in reader.lines().enumerate() {
                let line = line.unwrap().to_string();
                let tokens: Vec<&str> = line.split(' ').collect();
                if tokens.len() != 7 {
                    panic!("Line {} in {:?} is malformed", line_number + 1, path);
                }

                let cut = Card::from_str(tokens[0]).unwrap();
                let card1 = Card::from_str(tokens[1]).unwrap();
                let card2 = Card::from_str(tokens[2]).unwrap();
                let card3 = Card::from_str(tokens[3]).unwrap();
                let card4 = Card::from_str(tokens[4]).unwrap();
                let expected_non_crib_hand_score: u8 = tokens[5].parse().unwrap();
                let expected_crib_hand_score: u8 = tokens[6].parse().unwrap();

                let mut hand = Hand::new([card1, card2, card3, card4], cut, false);
                let non_crib_hand_score = hand.score();
                if non_crib_hand_score != expected_non_crib_hand_score {
                    panic!(
                        "Line {} in {:?}: {:?} scored {} points as a non-crib hand,\
                         but should have scored {} points",
                        line_number + 1,
                        path,
                        hand,
                        non_crib_hand_score,
                        expected_non_crib_hand_score
                    );
                }

                let mut crib_hand = Hand::new([card1, card2, card3, card4], cut, true);
                let crib_hand_score = crib_hand.score();
                if crib_hand_score != expected_crib_hand_score {
                    panic!(
                        "Line {} in {:?}: {:?} scored {} points as a crib hand,\
                         but should have scored {} points",
                        line_number + 1,
                        path,
                        hand,
                        non_crib_hand_score,
                        expected_non_crib_hand_score
                    );
                }
            }

            path.pop();
        }
    }
}
