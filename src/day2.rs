use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    direction: Direction,
    value: u64,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> u64 {
    let mut depth = 0;
    let mut horiz = 0;
    for cmd in input {
        let Command { direction, value } = cmd;
        match direction {
            Direction::Forward => horiz += value,
            Direction::Down => depth += value,
            Direction::Up => depth -= value,
        }
    }
    depth * horiz
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> u64 {
    let mut aim = 0;
    let mut depth = 0;
    let mut horiz = 0;

    for cmd in input {
        let Command { direction, value } = cmd;
        match direction {
            Direction::Forward => {
                horiz += value;
                depth += value * aim;
            }
            Direction::Down => aim += value,
            Direction::Up => aim -= value,
        }
    }

    depth * horiz
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(format!("unknown direction: {}", s)),
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value) = s.split_once(' ').ok_or("nothing to split")?;
        let direction: Direction = direction.parse()?;
        let value = value.parse::<u64>().map_err(|e| e.to_string())?;
        Ok(Command { direction, value })
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, Direction};

    const COMMANDS: [Command; 6] = [
        Command {
            direction: Direction::Forward,
            value: 5,
        },
        Command {
            direction: Direction::Down,
            value: 5,
        },
        Command {
            direction: Direction::Forward,
            value: 8,
        },
        Command {
            direction: Direction::Up,
            value: 3,
        },
        Command {
            direction: Direction::Down,
            value: 8,
        },
        Command {
            direction: Direction::Forward,
            value: 2,
        },
    ];

    #[test]
    fn generator() {
        let commands = super::input_generator(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2",
        );
        assert_eq!(commands, COMMANDS);
    }

    #[test]
    fn part1() {
        let result = super::part1(&COMMANDS);
        assert_eq!(result, 150);
    }

    #[test]
    fn part2() {
        let result = super::part2(&COMMANDS);
        assert_eq!(result, 900);
    }
}
