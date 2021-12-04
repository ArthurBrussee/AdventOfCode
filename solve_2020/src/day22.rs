use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

fn crab_game(p1: &VecDeque<u8>, p2: &VecDeque<u8>, recurse: bool) -> (u32, u32) {
    fn score(deck: &VecDeque<u8>) -> u32 {
        deck.iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| ((i + 1) as u32) * (x as u32))
            .sum()
    }

    let mut p1 = p1.clone();
    let mut p2 = p2.clone();

    let mut rounds: HashSet<VecDeque<u8>> = HashSet::new();

    let (winner, score) = loop {
        if !rounds.insert(p1.clone()) {
            return (0, score(&p1));
        }

        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();

        let p1_wins = if recurse && p1_card <= p1.len() as u8 && p2_card <= p2.len() as u8 {
            let p1_subdeck = VecDeque::from_iter(p1.iter().take(p1_card as usize).copied());
            let p2_subdeck = VecDeque::from_iter(p2.iter().take(p2_card as usize).copied());
            let (winner, _) = crab_game(&p1_subdeck, &p2_subdeck, recurse);
            winner == 0
        } else {
            p1_card > p2_card
        };

        if p1_wins {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }

        if p1.is_empty() {
            break (1, score(&p2));
        }
        if p2.is_empty() {
            break (0, score(&p1));
        }
    };
    (winner, score)
}

pub fn calc() -> (u32, u32) {
    let player1_deck: VecDeque<u8> = VecDeque::from(vec![
        1, 43, 24, 34, 13, 7, 10, 36, 14, 12, 47, 32, 11, 3, 9, 25, 37, 21, 2, 45, 26, 8, 23, 6, 49,
    ]);
    let player2_deck: VecDeque<u8> = VecDeque::from(vec![
        44, 5, 46, 18, 39, 50, 4, 41, 17, 28, 30, 42, 33, 38, 35, 22, 16, 27, 40, 48, 19, 29, 15,
        31, 20,
    ]);

    let (_, score_p1) = crab_game(&player1_deck, &player2_deck, false);
    let (_, score_p2) = crab_game(&player1_deck, &player2_deck, true);
    (score_p1, score_p2)
}
