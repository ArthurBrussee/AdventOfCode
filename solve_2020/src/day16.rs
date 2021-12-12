use std::ops::Range;

struct Condition {
    name: String,
    min: Range<u64>,
    max: Range<u64>,
}

pub fn calc(input: &str) -> (u64, u64) {
    let lines = input.lines().collect::<Vec<_>>();

    let constraints = lines
        .iter()
        .take(20)
        .map(|line| {
            let mut splits = line.split(':');
            let name = splits.next().unwrap();
            let mut ranges = splits.next().unwrap().split(" or ");
            let mut min_range = ranges
                .next()
                .unwrap()
                .split('-')
                .map(|x| x.trim().parse().unwrap());
            let mut max_range = ranges
                .next()
                .unwrap()
                .split('-')
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
        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>())
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
        let mut solution_cur = vec![0; valid_tickets[0].len()];

        let mut possible_set: Vec<Vec<usize>> = (0..valid_tickets[0].len())
            .map(|ind| {
                (0..constraints.len())
                    .filter(|constraint_ind| {
                        let constraint = &constraints[*constraint_ind];

                        valid_tickets.iter().all(|ticket| {
                            let ticket_num = ticket[ind];
                            constraint.min.contains(&ticket_num)
                                || constraint.max.contains(&ticket_num)
                        })
                    })
                    .collect()
            })
            .collect();

        while let Some(i) = possible_set.iter().position(|s| s.len() == 1) {
            let s = possible_set[i][0];
            for remaining in possible_set.iter_mut() {
                if let Some(j) = remaining.iter().position(|&x| x == s) {
                    remaining.swap_remove(j);
                }
            }
            solution_cur[i] = s;
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
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let p2 = my_ticket
        .iter()
        .enumerate()
        .map(|(ind, val)| {
            let index = solution[ind];
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
