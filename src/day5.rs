use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    start: Point,
    end: Point,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (l0, l1) = line.split_once(" -> ").unwrap();
            let start = l0.split_once(',').unwrap();
            let p0 = Point::new(
                start.0.parse::<u16>().unwrap(),
                start.1.parse::<u16>().unwrap(),
            );
            let end = l1.split_once(',').unwrap();
            let p1 = Point::new(end.0.parse::<u16>().unwrap(), end.1.parse::<u16>().unwrap());
            Line::new(p0, p1)
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(lines: &[Line]) -> u64 {
    let mut lines_at_point = HashMap::new();
    lines
        .iter()
        .flat_map(|line| line.all_points_on_line(false))
        .for_each(|point| {
            lines_at_point
                .entry(point)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
    lines_at_point
        .iter()
        .filter(|(_, &count)| count >= 2)
        .count()
        .try_into()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(lines: &[Line]) -> u64 {
    let mut lines_at_point = HashMap::new();
    lines
        .iter()
        .flat_map(|line| line.all_points_on_line(true))
        .for_each(|point| {
            lines_at_point
                .entry(point)
                .and_modify(|e| *e += 1)
                .or_insert(1_u16);
        });
    lines_at_point
        .iter()
        .filter(|(_, &count)| count >= 2)
        .count()
        .try_into()
        .unwrap()
}

impl Point {
    const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Line {
    const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn all_points_on_line(&self, with_diags: bool) -> Vec<Point> {
        let &Self {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        } = self;
        if x1 == x2 {
            range(y1, y2).map(|y| Point::new(x1, y)).collect()
        } else if y1 == y2 {
            range(x1, x2).map(|x| Point::new(x, y1)).collect()
        } else if with_diags {
            range(x1, x2)
                .zip(range(y1, y2))
                .map(|(x, y)| Point::new(x, y))
                .collect()
        } else {
            vec![]
        }
    }
}

fn range(start: u16, end: u16) -> Box<dyn Iterator<Item = u16>> {
    if start < end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Point};

    const RAW_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    const LINES: [Line; 10] = [
        Line::new(Point::new(0, 9), Point::new(5, 9)),
        Line::new(Point::new(8, 0), Point::new(0, 8)),
        Line::new(Point::new(9, 4), Point::new(3, 4)),
        Line::new(Point::new(2, 2), Point::new(2, 1)),
        Line::new(Point::new(7, 0), Point::new(7, 4)),
        Line::new(Point::new(6, 4), Point::new(2, 0)),
        Line::new(Point::new(0, 9), Point::new(2, 9)),
        Line::new(Point::new(3, 4), Point::new(1, 4)),
        Line::new(Point::new(0, 0), Point::new(8, 8)),
        Line::new(Point::new(5, 5), Point::new(8, 2)),
    ];

    #[test]
    fn parse() {
        let lines = super::input_generator(RAW_INPUT);
        assert_eq!(lines, LINES);
    }

    #[test]
    fn part1() {
        let lines = super::input_generator(RAW_INPUT);
        assert_eq!(super::part1(&lines), 5);
    }

    #[test]
    fn part2() {
        let lines = super::input_generator(RAW_INPUT);
        assert_eq!(super::part2(&lines), 12);
    }
}
