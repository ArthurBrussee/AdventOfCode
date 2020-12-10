use std::fs;

pub fn calc() -> (i64, i64) {
    let mut nums = fs::read_to_string("./inputs/day10.txt")
        .expect("Can't find input file.")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    nums.push(0);
    nums.sort();
    let device = nums.last().unwrap() + 3;
    nums.push(device);

    let (count1, count3) =
        nums.iter()
            .zip(nums.iter().skip(1))
            .fold((0, 0), |(count1, count3), (low, hi)| {
                let dif = hi - low;
                match dif {
                    1 => (count1 + 1, count3),
                    3 => (count1, count3 + 1),
                    _ => (count1, count3),
                }
            });

    let p1 = count1 * count3;

    let mut path_count: Vec<i64> = vec![0; device as usize + 1];

    path_count[0] = 1;
    for &n in &nums[1..] {
        path_count[n as usize] = path_count.get((n - 1) as usize).unwrap_or(&0)
            + path_count.get((n - 2) as usize).unwrap_or(&0)
            + path_count.get((n - 3) as usize).unwrap_or(&0);
    }

    let p2 = path_count[device as usize];
    (p1, p2)
}
