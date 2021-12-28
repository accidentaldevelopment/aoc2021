#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct CrabSubmarine {
    h_pos: u64,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<CrabSubmarine> {
    input
        .split(',')
        .map(|s| CrabSubmarine::new(s.parse().unwrap()))
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[CrabSubmarine]) -> u64 {
    let positions: Vec<_> = input.iter().map(|s| s.h_pos).collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|pos| {
            input.iter().fold(0, |acc, c| {
                let diff = if pos > c.h_pos {
                    pos - c.h_pos
                } else {
                    c.h_pos - pos
                };
                acc + diff
            })
        })
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[CrabSubmarine]) -> u64 {
    let positions: Vec<_> = input.iter().map(|s| s.h_pos).collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|pos| {
            input.iter().fold(0, |acc, c| {
                let diff = if pos > c.h_pos {
                    pos - c.h_pos
                } else {
                    c.h_pos - pos
                };
                acc + (diff * (diff + 1) / 2)
            })
        })
        .min()
        .unwrap()
}

impl CrabSubmarine {
    fn new(h_pos: u64) -> Self {
        Self { h_pos }
    }
}

#[cfg(test)]
mod tests {
    use super::CrabSubmarine;

    const RAW_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn input_generator() {
        assert_eq!(
            super::input_generator(RAW_INPUT),
            [
                CrabSubmarine::new(16),
                CrabSubmarine::new(1),
                CrabSubmarine::new(2),
                CrabSubmarine::new(0),
                CrabSubmarine::new(4),
                CrabSubmarine::new(2),
                CrabSubmarine::new(7),
                CrabSubmarine::new(1),
                CrabSubmarine::new(2),
                CrabSubmarine::new(14),
            ]
        );
    }

    #[test]
    fn part1() {
        let subs = super::input_generator(RAW_INPUT);
        assert_eq!(super::part1(&subs), 37);
    }

    #[test]
    fn part2() {
        let subs = super::input_generator(RAW_INPUT);
        assert_eq!(super::part2(&subs), 168);
    }
}
