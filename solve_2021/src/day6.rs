use aoc_lib::AocSolution;

fn simulate_fish(populations: &mut [usize; 9]) {
    let breed_pop = populations[0];

    for i in 1..=8 {
        populations[i - 1] = populations[i];
    }

    // Fish reset to 6
    populations[6] += breed_pop;
    // And breed new fish with timer of 8.
    populations[8] = breed_pop;
}

fn create_pop(day_counts: &[usize]) -> [usize; 9] {
    let mut pop = [0; 9];
    for &d in day_counts {
        pop[d] += 1;
    }
    pop
}

pub struct Solution;

impl AocSolution<usize, usize> for Solution {
    const DATE: (u32, u32) = (2021, 6);

    fn calc(input: &str) -> (usize, usize) {
        let nums: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
        let mut pop = create_pop(&nums);
        for _ in 0..80 {
            simulate_fish(&mut pop);
        }
        let p1 = pop.iter().sum();
        for _ in 80..256 {
            simulate_fish(&mut pop);
        }
        let p2 = pop.iter().sum();
        (p1, p2)
    }
}

#[test]
fn test() {
    Solution::test(5934, 26984457539);
}
