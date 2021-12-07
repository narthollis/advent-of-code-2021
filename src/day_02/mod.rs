pub enum Move {
    Up(u32),
    Down(u32),
    Forward(u32),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| -> (&str, u32) {
            let mut bits = l.trim().split(' ');
            (bits.next().unwrap(), bits.next().unwrap().parse().unwrap())
        })
        .map(|(d, v)| -> Move {
            match d {
                "forward" => Move::Forward(v),
                "up" => Move::Up(v),
                "down" => Move::Down(v),
                _ => panic!("unexpected direction")
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Move]) -> u32 {
    let mut depth = 0;
    let mut distance = 0;

    for m in input {
        match m {
            Move::Up(v) => depth -= v,
            Move::Down(v) => depth += v,
            Move::Forward(v) => distance += v,
        }
    }

    depth * distance
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Move]) -> u32 {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    for m in input {
        match m {
            Move::Up(v) => aim -= v,
            Move::Down(v) => aim += v,
            Move::Forward(v) => {
                distance += v;
                depth += aim * v;
            }
        }
    }

    depth * distance
}

