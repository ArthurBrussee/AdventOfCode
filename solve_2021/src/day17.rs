use std::ops::RangeInclusive;

fn sim(vel: (i32, i32), xr: RangeInclusive<i32>, yr: RangeInclusive<i32>) -> Option<i32> {
    let mut pos = (0, 0);
    let mut maxy = 0;
    let mut vel = vel;

    loop {
        pos.0 += vel.0;
        pos.1 += vel.1;

        vel.0 -= vel.0.signum();
        vel.1 -= 1;

        maxy = maxy.max(pos.1);

        if xr.contains(&pos.0) && yr.contains(&pos.1) {
            break Some(maxy);
        }

        if pos.1 < *yr.start() && vel.1 < 0 {
            break None;
        }
    }
}

pub fn calc(input: &str) -> (i32, usize) {
    let (xr, yr) = input.split_once(' ').unwrap();
    let (xr, yr) = (xr.split_once(',').unwrap(), yr.split_once(',').unwrap());
    let (xr, yr) = (
        (xr.0.parse().unwrap(), xr.1.parse().unwrap()),
        (yr.0.parse().unwrap(), yr.1.parse().unwrap()),
    );

    let mut max_maxy = 0;
    let mut hits = 0;

    for xvel in 0..=xr.1 {
        for yvel in -250..250 {
            if let Some(maxy) = sim((xvel, yvel), xr.0..=xr.1, yr.0..=yr.1) {
                max_maxy = max_maxy.max(maxy);
                hits += 1;
            }
        }
    }

    (max_maxy, hits)
}

#[test]
fn test() {
    let (p1, p2) = calc("20,30 -10,-5");
    assert_eq!(p1, 45);
    assert_eq!(p2, 112);
}
