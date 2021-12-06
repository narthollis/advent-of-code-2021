#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut iter = input.iter();
    let mut prev = *iter.next().unwrap();

    iter.fold(0, |a, &v| {
        let next = if prev < v { a + 1 } else { a };
        prev = v;
        return next;
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut iter = input.windows(3).map(|i| i.iter().map(|&v| v).sum());
    let mut prev: u32 = iter.next().unwrap();

    iter.fold(0, |a, v| {
        let next = if prev < v { a + 1 } else { a };
        prev = v;
        return next;
    })
}