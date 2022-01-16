fn mod_nat(num: usize, n: usize) -> usize {
    (num - 1) % n + 1
}

fn normal_die(spaces: [usize; 2]) -> usize {
    let mut spaces = spaces;
    let mut die = 1;
    let mut total_rolls = 0;

    let mut scores = [0; 2];

    loop {
        let player = total_rolls % 2;
        let roll = die * 3 + 3;

        spaces[player] = mod_nat(spaces[player] + roll, 10);
        scores[player] += spaces[player];
        total_rolls += 1;
        die = mod_nat(die + 3, 100);

        if scores[0] >= 1000 || scores[1] >= 1000 {
            break scores.iter().min().unwrap() * total_rolls * 3;
        }
    }
}

fn quantum_die(player: usize, positions: [usize; 2], scores: [usize; 2]) -> [usize; 2] {
    if scores[0] >= 21 {
        return [1, 0];
    }

    if scores[1] >= 21 {
        return [0, 1];
    }

    let player = if player == 0 { 1 } else { 0 };

    let die_rolls = [3, 4, 5, 6, 7, 8, 9];
    let die_weights = [1, 3, 6, 7, 6, 3, 1];

    let mut branches = [0, 0];

    for (roll, weight) in die_rolls.into_iter().zip(die_weights) {
        let mut new_position = positions;
        new_position[player] = mod_nat(new_position[player] + roll, 10);

        let mut new_score = scores;
        new_score[player] += new_position[player];

        let subtree = quantum_die(player, new_position, new_score);

        branches = [
            branches[0] + weight * subtree[0],
            branches[1] + weight * subtree[1],
        ];
    }

    branches
}

pub fn calc(input: &str) -> (usize, usize) {
    let (p1_str, p2_str) = input.split_once(',').unwrap();
    let spaces = [p1_str.parse().unwrap(), p2_str.parse().unwrap()];
    let p1 = normal_die(spaces);
    let wins = quantum_die(1, spaces, [0, 0]);
    let p2 = *wins.iter().max().unwrap();
    (p1, p2)
}

#[test]
fn test() {
    let (p1, p2) = calc("4,8");
    assert_eq!(p1, 739785);
    assert_eq!(p2, 444356092776315);
}
