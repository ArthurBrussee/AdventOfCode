use aoc_lib::AocSolution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Weed {
    Empty,
    East,
    South,
}

#[derive(Debug)]
struct Map {
    state: Vec<Weed>,
    width: usize,
    height: usize,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let vals = input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '>' => Weed::East,
                    'v' => Weed::South,
                    _ => Weed::Empty,
                })
            })
            .collect();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        Map {
            state: vals,
            width,
            height,
        }
    }
}

impl Map {
    fn index(&self, x: usize, y: usize) -> usize {
        let x_wrapped = x % self.width;
        let y_wrapped = y % self.height;
        x_wrapped + y_wrapped * self.width
    }

    fn step(&mut self) -> bool {
        let mut changed = false;

        self.state = {
            let mut new_state = self.state.clone();
            for (idx, w) in self.state.iter().copied().enumerate() {
                let x = idx % self.width;
                let y = idx / self.width;
                let nidx = self.index(x + 1, y);

                if (w, self.state[nidx]) == (Weed::East, Weed::Empty) {
                    new_state.swap(idx, nidx);
                    changed = true;
                }
            }
            new_state
        };

        self.state = {
            let mut new_state = self.state.clone();

            for (idx, w) in self.state.iter().copied().enumerate() {
                let x = idx % self.width;
                let y = idx / self.width;
                let nidx = self.index(x, y + 1);

                if (w, self.state[nidx]) == (Weed::South, Weed::Empty) {
                    new_state.swap(idx, nidx);
                    changed = true;
                }
            }
            new_state
        };

        changed
    }
}

pub struct Solution;

impl AocSolution<usize, String> for Solution {
    const DATE: (u32, u32) = (2021, 25);

    fn calc(input: &str) -> (usize, String) {
        let mut map = Map::from(input);

        let p1 = (1..)
            .map(|i| (i, map.step()))
            .find(|(_, change)| !change)
            .unwrap()
            .0;

        (p1, "Merry xmas".to_owned())
    }
}

#[test]
fn test() {
    let (p1, _) = Solution::calc(&Solution::read_file(true));
    assert_eq!(p1, 58);
}
