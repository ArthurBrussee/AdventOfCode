use std::fs;

#[derive(Debug, Clone, Copy)]
struct Slot {
    num: u32,
    called: bool,
}

#[derive(Clone, Copy)]
struct Board {
    slots: [Slot; 25],
}

impl Board {
    fn try_mark(&mut self, num: u32) {
        if let Some(index) = self.slots.iter().position(|r| r.num == num) {
            self.slots[index].called = true;
        }
    }

    fn is_winning(&self) -> bool {
        for x in 0..5 {
            for y in 0..5 {
                let mut row_set = true;

                for dx in 0..5 {
                    if !self.is_called(dx, y as u32) {
                        row_set = false;
                    }
                }

                let mut column_set = true;
                for dy in 0..5 {
                    if !self.is_called(x as u32, dy) {
                        column_set = false;
                    }
                }

                if row_set || column_set {
                    return true;
                }
            }
        }

        false
    }

    fn unmarked_score(&self) -> u32 {
        self.slots.iter().filter(|s| !s.called).map(|s| s.num).sum()
    }

    fn is_called(&self, x: u32, y: u32) -> bool {
        self.slots[(x + y * 5) as usize].called
    }
}

fn load_input(path: &str) -> (Vec<Board>, Vec<u32>) {
    let input = fs::read_to_string(path).unwrap();
    let mut parts = input.split("\n\n");

    let first_line = parts.next().unwrap();
    let nums: Vec<u32> = first_line.split(',').map(|x| x.parse().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();

    for part in parts {
        let mut board_nums: Vec<u32> = Vec::new();

        for l in part.lines() {
            let nums = l.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            board_nums.extend(nums);
        }

        let slots: Vec<Slot> = board_nums
            .iter()
            .map(|&num| Slot { num, called: false })
            .collect();

        boards.push(Board {
            slots: slots.try_into().unwrap(),
        });
    }

    (boards, nums)
}

fn solve_winning_score(boards: &mut Vec<Board>, numbers: &[u32]) -> u32 {
    for &num in numbers {
        for board in boards.iter_mut() {
            board.try_mark(num);

            if board.is_winning() {
                return num * board.unmarked_score();
            }
        }
    }

    panic!("Noone won...");
}

fn last_winning_score(boards: &mut Vec<Board>, numbers: &[u32]) -> u32 {
    for &num in numbers {
        for board in boards.iter_mut() {
            board.try_mark(num);
        }

        if boards.len() == 1 && boards[0].is_winning() {
            return boards[0].unmarked_score() * num;
        }

        boards.retain(|b| !b.is_winning());
    }

    panic!("Noone won...");
}

pub fn calc() -> (u32, u32) {
    let (boards, numbers) = load_input("./solve_2021/inputs/day4.txt");

    let mut boards1 = boards.to_vec();
    let mut boards2 = boards.to_vec();

    (
        solve_winning_score(&mut boards1, &numbers),
        last_winning_score(&mut boards2, &numbers),
    )
}

#[test]
fn test_p1() {
    let (mut boards, numbers) = load_input("./inputs/day4_test.txt");
    assert_eq!(solve_winning_score(&mut boards, &numbers), 4512);
}

#[test]
fn test_p2() {
    let (mut boards, numbers) = load_input("./inputs/day4_test.txt");
    assert_eq!(last_winning_score(&mut boards, &numbers), 1924);
}
