fn get_spoken_num(nums: &[u32], max_turn: u32) -> u32 {
    let mut last = 0;
    let mut last_turn = vec![0u32; max_turn as usize];

    for (turn, n) in nums.iter().enumerate() {
        last_turn[*n as usize] = turn as u32 + 1;
        last = *n;
    }

    for turn in nums.len() as u32..max_turn {
        let last_spoken = &mut last_turn[last as usize];
        let last_turn = *last_spoken;
        *last_spoken = turn;
        last = if last_turn == 0 { 0 } else { turn - last_turn };
    }
    last
}

pub fn calc(_: &str) -> (u32, u32) {
    let nums: Vec<u32> = vec![15, 5, 1, 4, 7, 0];
    (get_spoken_num(&nums, 2020), get_spoken_num(&nums, 30000000))
}
