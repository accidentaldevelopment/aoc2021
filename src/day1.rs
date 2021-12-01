#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().expect("parse error"))
        .collect()
}

#[aoc(day1, part1, for_loop)]
pub fn part1_loop(input: &[u32]) -> u32 {
    let mut increases = 0;
    let mut last = input[0];
    for depth in input {
        if depth > &last {
            increases += 1;
        }
        last = *depth;
    }
    increases
}

#[aoc(day1, part1, fold)]
pub fn part1_fold(input: &[u32]) -> u32 {
    let mut last = input[0];
    input.iter().fold(0_u32, |mut i, &c| {
        if c > last {
            i += 1;
        }
        last = c;
        i
    })
}

#[aoc(day1, part2, for_loop)]
pub fn part2_loop(input: &[u32]) -> u32 {
    let mut last: u32 = input.windows(3).next().unwrap().iter().sum();
    let mut increases = 0;

    for i in input.windows(3) {
        let sum = i.iter().sum();

        if sum > last {
            increases += 1;
        }

        last = sum;
    }

    increases
}

#[aoc(day1, part2, fold)]
pub fn part2_fold(input: &[u32]) -> u32 {
    let mut last: u32 = input.windows(3).next().unwrap().iter().sum();

    input.windows(3).fold(0, |mut increases, n| {
        let sum = n.iter().sum();
        if sum > last {
            increases += 1;
        }
        last = sum;
        increases
    })
}

// Try out part1 again with this weird struct to handle state

struct IncreaseCounter {
    last: u32,
    count: u32,
}

impl IncreaseCounter {
    fn new(initial: u32) -> Self {
        Self {
            last: initial,
            count: 0,
        }
    }
    fn submit(&mut self, i: u32) {
        if i > self.last {
            self.count += 1;
        }
        self.last = i;
    }
}

#[aoc(day1, part1, weird_struct_loop)]
pub fn part1_weird_struct_loop(input: &[u32]) -> u32 {
    let mut increase_counter = IncreaseCounter::new(input[0]);
    for depth in input {
        increase_counter.submit(*depth);
    }
    increase_counter.count
}

#[aoc(day1, part1, weird_struct_functional)]
pub fn part1_weird_struct_functional(input: &[u32]) -> u32 {
    let mut increase_counter = IncreaseCounter::new(input[0]);
    input.iter().for_each(|i| increase_counter.submit(*i));
    increase_counter.count
}

#[cfg(test)]
mod tests {
    const TEST_DATA: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part1_loop() {
        let increases = super::part1_loop(&TEST_DATA);
        assert_eq!(increases, 7);
    }

    #[test]
    fn part1_fold() {
        let increases = super::part1_fold(&TEST_DATA);
        assert_eq!(increases, 7);
    }

    #[test]
    fn part2_loop() {
        let increases = super::part2_loop(&TEST_DATA);
        assert_eq!(increases, 5);
    }
}
