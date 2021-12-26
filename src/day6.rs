// #[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Default, Clone)]
pub struct Fish([u64; 9]);

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Fish {
    let mut fish = Fish::default();
    input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|i| fish.0[i] += 1);
    fish
}

#[aoc(day6, part1)]
pub fn part1(fish: &Fish) -> u64 {
    let mut fish = fish.clone();
    fish.advance_days(80);
    fish.count()
}

#[aoc(day6, part2)]
pub fn part2(fish: &Fish) -> u64 {
    let mut fish = fish.clone();
    fish.advance_days(256);
    fish.count()
}

impl Fish {
    fn advance_days(&mut self, num_days: u16) {
        for _ in 0..num_days {
            self.0.rotate_left(1);
            self.0[6] += self.0[8];
        }
    }

    fn count(&self) -> u64 {
        self.0.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    const RAW_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn input_generator() {
        let fish = super::input_generator(RAW_INPUT);
        assert_eq!(fish.0, [0, 1, 1, 2, 1, 0, 0, 0, 0,]);
    }

    #[test]
    fn part1() {
        let fish = super::input_generator(RAW_INPUT);
        assert_eq!(super::part1(&fish), 5934);
    }

    #[test]
    fn part2() {
        let fish = super::input_generator(RAW_INPUT);
        assert_eq!(super::part2(&fish), 26_984_457_539);
    }
}
