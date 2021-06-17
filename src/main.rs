use std::io::{stdin, stdout, Write};

use cribbage_core::{deal_two_player_hand, standard_two_player_board, Deck, Pegger};

pub fn main() {
    let board = standard_two_player_board();
    let mut deck = Deck::new();
    let _pegger = Pegger::new();

    while board.winner().is_none() {
        deck.shuffle();

        let player_one_deal = deal_two_player_hand(&mut deck).unwrap();
        let player_two_deal = deal_two_player_hand(&mut deck).unwrap();

        println!("Player 1 Cards:");
        for (index, card) in player_one_deal.cards().iter().enumerate() {
            println!("{}: {:?} ", index, card);
        }

        print!("Please select 4 cards to keep (by 0 index): ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input.");
        let indices: Vec<usize> = input
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        let keep = [
            player_one_deal.cards()[indices[0]],
            player_one_deal.cards()[indices[1]],
            player_one_deal.cards()[indices[2]],
            player_one_deal.cards()[indices[3]],
        ];

        print!("Please select 2 cards to move into crib (by 0 index): ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input.");
        let indices: Vec<usize> = input
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        let crib = [
            player_one_deal.cards()[indices[0]],
            player_one_deal.cards()[indices[1]],
        ];

        let (player_one_kept, player_one_crib) = player_one_deal.split(keep, crib);

        println!("Player 2 Cards:");
        for (index, card) in player_two_deal.cards().iter().enumerate() {
            println!("{}: {:?} ", index, card);
        }

        print!("Please select 4 cards to keep (by 0 index): ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input.");
        let indices: Vec<usize> = input
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        let keep = [
            player_two_deal.cards()[indices[0]],
            player_two_deal.cards()[indices[1]],
            player_two_deal.cards()[indices[2]],
            player_two_deal.cards()[indices[3]],
        ];

        print!("Please select 2 cards to move into crib (by 0 index): ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input.");
        let indices: Vec<usize> = input
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        let crib = [
            player_two_deal.cards()[indices[0]],
            player_two_deal.cards()[indices[1]],
        ];
        let (player_two_kept, player_two_crib) = player_two_deal.split(keep, crib);

        let crib = player_one_crib.combine(player_two_crib);

        print!("Player one kept: ");
        for card in player_one_kept.cards() {
            print!("{:?} ", card);
        }
        println!();

        print!("Player two kept: ");
        for card in player_two_kept.cards() {
            print!("{:?} ", card);
        }
        println!();

        print!("Crib: ");
        for card in crib.cards() {
            print!("{:?} ", card);
        }
        println!();

        let cut = deck.draw().unwrap();
        println!("Cut card: {:?}", cut);

        let mut player_one_hand = player_one_kept.add_cut_card(cut);
        let mut player_two_hand = player_two_kept.add_cut_card(cut);
        let mut crib = crib.add_cut_card(cut);

        print!("Player one hand: ");
        for card in player_one_hand.cards() {
            print!("{:?} ", card);
        }
        println!("Player one hand points: {}", &player_one_hand.score());

        print!("Player two hand: ");
        for card in player_two_hand.cards() {
            print!("{:?} ", card);
        }
        println!("Player two hand points: {}", player_two_hand.score());

        print!("Crib: ");
        for card in crib.cards() {
            print!("{:?} ", card);
        }
        println!("Crib points: {}", crib.score());
    }
}
