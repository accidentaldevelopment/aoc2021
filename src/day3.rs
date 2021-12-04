#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<bool>]) -> u64 {
    let mut bit_counts = vec![0; input.get(0).unwrap().len()];
    for line in input {
        for (pos, &bit) in line.iter().enumerate() {
            if bit {
                bit_counts[pos] += 1;
            } else {
                bit_counts[pos] -= 1;
            }
        }
    }
    let (gamma, eps) = bit_counts.iter().fold((0, 0), |(mut gamma, mut eps), &b| {
        if b > 0 {
            gamma = (gamma << 1) | 1;
            eps <<= 1;
        } else {
            gamma <<= 1;
            eps = (eps << 1) | 1;
        }
        (gamma, eps)
    });

    gamma * eps
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<bool>]) -> u64 {
    let o2_generator = winnow(input, true)
        .iter()
        .fold(0_u64, |i, &b| if b { i << 1 | 1 } else { i << 1 });
    let co2_scrubber = winnow(input, false)
        .iter()
        .fold(0_u64, |i, &b| if b { i << 1 | 1 } else { i << 1 });

    o2_generator * co2_scrubber
}

fn count_bits_at_pos(input: &[Vec<bool>], pos: usize) -> i32 {
    input.iter().fold(0, |counter, v| {
        counter + if *v.get(pos).unwrap() { 1 } else { -1 }
    })
}

fn winnow(input: &[Vec<bool>], most_common: bool) -> Vec<bool> {
    let len = input.get(0).unwrap().len();
    let mut input = input.to_vec();
    let mut pos = 0;
    let mut want;
    while input.len() != 1 {
        let bit_count = count_bits_at_pos(&input, pos);
        want = if most_common {
            bit_count >= 0
        } else {
            bit_count < 0
        };
        input = input
            .into_iter()
            .filter(|i| *i.get(pos).unwrap() == want)
            .collect();
        pos = (pos + 1) % len;
    }
    input.get(0).unwrap().clone()
}

// #[aoc(day3, part2)]
// pub fn part2(_input: &[u64]) -> u64 {
//     0
// }

#[cfg(test)]
mod tests {
    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn generator() {
        let result = super::input_generator("00100\n11110");
        assert_eq!(
            result,
            [
                [false, false, true, false, false],
                [true, true, true, true, false]
            ]
        );
    }

    #[test]
    fn part1() {
        let input = super::input_generator(INPUT);
        assert_eq!(super::part1(&input), 198);
    }

    #[test]
    fn count_bits_at_pos() {
        let input = super::input_generator(INPUT);
        assert_eq!(super::count_bits_at_pos(&input, 1), -2);
    }

    #[test]
    fn winnow() {
        // most common bit
        let input = super::input_generator(INPUT);
        assert_eq!(super::winnow(&input, true), [true, false, true, true, true]);

        // least common bit
        let input = super::input_generator(INPUT);
        assert_eq!(
            super::winnow(&input, false),
            [false, true, false, true, false]
        );
    }

    #[test]
    fn part2() {
        let input = super::input_generator(INPUT);
        assert_eq!(super::part2(&input), 230);
    }
}
