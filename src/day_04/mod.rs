use array2d::Array2D;

pub struct Input {
    picks: Vec<u32>,
    boards: Vec<Array2D<u32>>,
}

trait Diagonals<T> {

    fn columns_iter(&self) -> Iterator<&T>;
}


#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    
    let picks: Vec<u32> = lines.next().unwrap()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let boards = lines.filter(|l| l.trim() != "")
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|g| {
            let iter = g.iter()
                .flat_map(|l| l.trim().split(' '))
                .filter(|i| i.trim() != "")
                .map(|i| i.parse().unwrap());

            Array2D::from_iter_row_major(iter, 5, 5)
        })
        .collect();


    Input { picks, boards }
}

#[aoc(day4, part1)]
pub fn sovler_part1(input: &Input) -> u32 {
    println!("{:?}", input.picks);
    println!("{:?}", input.boards);

    0
}