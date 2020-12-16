use std::{fs, ops::Range};

struct Condition {
    name: String,
    min: Range<u64>,
    max: Range<u64>,
}

pub fn calc() -> (u64, u64) {
    let file_str = fs::read_to_string("./inputs/day16.txt").expect("Can't find input file.");
    let lines = file_str.lines().collect::<Vec<_>>();

    let constraints = lines
        .iter()
        .take(20)
        .map(|line| {
            let mut splits = line.split(":");
            let name = splits.next().unwrap();
            let mut ranges = splits.next().unwrap().split(" or ");
            let mut min_range = ranges
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.trim().parse().unwrap());
            let mut max_range = ranges
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse().unwrap());

            Condition {
                name: name.to_string(),
                min: Range {
                    start: min_range.next().unwrap(),
                    end: min_range.next().unwrap() + 1,
                },
                max: Range {
                    start: max_range.next().unwrap(),
                    end: max_range.next().unwrap() + 1,
                },
            }
        })
        .collect::<Vec<_>>();

    let other_tickets = lines
        .iter()
        .skip(25)
        .map(|l| l.split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let valid_tickets = other_tickets
        .iter()
        .filter(|ticket| {
            !ticket.iter().any(|n| {
                constraints
                    .iter()
                    .all(|c| !c.min.contains(n) && !c.max.contains(n))
            })
        })
        .collect::<Vec<_>>();

    let solution = {
        let mut possible_set: Vec<Vec<usize>> = (0..valid_tickets[0].len())
            .map(|_| (0..constraints.len()).collect())
            .collect();
        let mut solution_cur = vec![None; valid_tickets[0].len()];

        while solution_cur.iter().any(|x| x.is_none()) {
            let mut solved_any = false;

            for (ind, remaining) in possible_set.iter_mut().enumerate() {
                remaining.retain(|constraint_ind| {
                    let constraint = &constraints[*constraint_ind];

                    valid_tickets.iter().all(|ticket| {
                        let ticket_num = ticket[ind];
                        constraint.min.contains(&ticket_num) || constraint.max.contains(&ticket_num)
                    })
                });

                if remaining.len() == 1 {
                    solved_any = true;
                }
            }

            while solved_any {
                solved_any = false;

                for ind in 0..possible_set.len() {
                    let set = &possible_set[ind];

                    if set.len() == 1 {
                        let s = set[0];
                        for remaining in possible_set.iter_mut() {
                            remaining.retain(|&r| r != s);

                            if remaining.len() == 1 {
                                solved_any = true;
                            }
                        }
                        solution_cur[ind] = Some(s);
                    }
                }
            }
        }
        solution_cur
    };

    let p1 = other_tickets
        .iter()
        .flat_map(|ticket| {
            ticket.iter().filter(|n| {
                constraints
                    .iter()
                    .all(|c| !c.min.contains(n) && !c.max.contains(n))
            })
        })
        .sum();

    let my_ticket = lines[22]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let p2 = my_ticket
        .iter()
        .enumerate()
        .map(|(ind, val)| {
            let index = solution[ind].unwrap();
            (&constraints[index].name, val)
        })
        .filter_map(|(name, val)| {
            if name.starts_with("departure") {
                Some(val)
            } else {
                None
            }
        })
        .product();

    (p1, p2)
}
