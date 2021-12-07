#[aoc_generator(day3)]
pub fn input_generator_day3(input: &str) -> Vec<[bool; 12]> {
    input
        .lines()
        .map(|l| -> [bool; 12] {
            let mut v = l.chars().map(|c| c == '1');
            [
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
            ]
        })
        .collect()
}

fn get_most_common(input: &Vec<[bool; 12]>) -> [bool; 12] {
    let mut high_count: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut low_count: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for entry in input {
        let mut idx = 0;
        for &v in entry {
            match v {
                true => high_count[idx] += 1,
                false => low_count[idx] += 1,
            }

            idx += 1;
        }
    }

    [
        high_count[0] >= low_count[0],
        high_count[1] >= low_count[1],
        high_count[2] >= low_count[2],
        high_count[3] >= low_count[3],
        high_count[4] >= low_count[4],
        high_count[5] >= low_count[5],
        high_count[6] >= low_count[6],
        high_count[7] >= low_count[7],
        high_count[8] >= low_count[8],
        high_count[9] >= low_count[9],
        high_count[10] >= low_count[10],
        high_count[11] >= low_count[11],
    ]
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &[[bool; 12]]) -> u32 {
    let highs = get_most_common(&input.to_vec());

    highs.iter().fold(0, |a, &v| a << 1 | (!v as u32)) * highs.iter().fold(0, |a, &v| a << 1 | (v as u32))    
}



#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &[[bool; 12]]) -> u32 {
    let mut pos = 0;
    let mut oxy = input.to_vec();
    while oxy.len() > 1 {
        oxy = oxy.iter()
            .filter(|&&i| i[pos] == get_most_common(&oxy)[pos])
            .map(|&i| i)
            .collect();

        pos +=1
    }

    pos = 0;
    let mut co2 = input.to_vec();
    while co2.len() > 1 {
        co2 = co2.iter()
            .filter(|&&i| i[pos] != get_most_common(&co2)[pos])
            .map(|&i| i)
            .collect();

        pos +=1
    }

    let oxy_int = oxy[0].iter().fold(0, |a, &v| a << 1 | (v as u32)); 
    let co2_int = co2[0].iter().fold(0, |a, &v| a << 1 | (v as u32));

    println!("oxy {:?} {:?}", oxy, oxy_int);
    println!("co2 {:?} {:?}", co2, co2_int);

    return oxy_int * co2_int;
}