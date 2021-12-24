#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    val: u8,
    marked: bool,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Board {
    squares: [[Square; 5]; 5],
    has_won: bool,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut input = input.split("\n\n");
    let calls = input
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = vec![];
    for data in input {
        boards.push(Board::parse(data));
    }

    (calls, boards)
}

#[aoc(day4, part1)]
pub fn part1((moves, boards): &(Vec<u8>, Vec<Board>)) -> u64 {
    let mut boards = boards.clone();

    for drawn in moves {
        for board in &mut boards {
            if board.mark_and_check_win(*drawn) {
                return board.sum_unmarked() * u64::from(*drawn);
            }
        }
    }
    panic!("no winner found")
}

#[aoc(day4, part2)]
pub fn part2((moves, boards): &(Vec<u8>, Vec<Board>)) -> u64 {
    let mut boards = boards.clone();
    let mut last_won_score = 0_u64;

    for drawn in moves {
        for board in &mut boards {
            if board.has_won {
                continue;
            }
            if board.mark_and_check_win(*drawn) {
                last_won_score = board.sum_unmarked() * u64::from(*drawn);
            }
        }
    }

    last_won_score
}

impl Board {
    fn parse<S: AsRef<str>>(input: S) -> Self {
        let mut squares: [[Square; 5]; 5] = Default::default();
        let mut input = input.as_ref().lines();

        for row in &mut squares {
            for (col, val) in input
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .enumerate()
            {
                row[col] = Square::new(val);
            }
        }

        Self {
            squares,
            has_won: false,
        }
    }

    fn mark_and_check_win(&mut self, val: u8) -> bool {
        let Self { squares, .. } = self;

        for row in 0..squares.len() {
            for col in 0..squares[row].len() {
                let mut square = &mut squares[row][col];
                if square.val == val {
                    square.marked = true;
                    if (0..squares.len()).all(|row| squares[row][col].marked)
                        || (0..squares[row].len()).all(|col| squares[row][col].marked)
                    {
                        self.has_won = true;
                        return true;
                    }
                }
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u64 {
        self.squares.iter().flatten().fold(0, |mut acc, sq| {
            if !sq.marked {
                acc += u64::from(sq.val);
            }
            acc
        })
    }
}

impl Square {
    fn new(val: u8) -> Self {
        Self { val, marked: false }
    }
}

#[cfg(test)]
mod tests {
    const RAW_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    const MOVES: [u8; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    #[test]
    fn parse() {
        let (moves, boards) = super::input_generator(RAW_INPUT);
        assert_eq!(moves, MOVES);
        assert_eq!(
            boards[0].squares,
            [
                [
                    super::Square::new(22),
                    super::Square::new(13),
                    super::Square::new(17),
                    super::Square::new(11),
                    super::Square::new(0),
                ],
                [
                    super::Square::new(8),
                    super::Square::new(2),
                    super::Square::new(23),
                    super::Square::new(4),
                    super::Square::new(24),
                ],
                [
                    super::Square::new(21),
                    super::Square::new(9),
                    super::Square::new(14),
                    super::Square::new(16),
                    super::Square::new(7),
                ],
                [
                    super::Square::new(6),
                    super::Square::new(10),
                    super::Square::new(3),
                    super::Square::new(18),
                    super::Square::new(5),
                ],
                [
                    super::Square::new(1),
                    super::Square::new(12),
                    super::Square::new(20),
                    super::Square::new(15),
                    super::Square::new(19),
                ],
            ]
        );
    }

    #[test]
    fn part1() {
        let (moves, boards) = super::input_generator(RAW_INPUT);
        assert_eq!(super::part1(&(moves, boards)), 4512);
    }

    #[test]
    fn part2() {
        let (moves, boards) = super::input_generator(RAW_INPUT);
        assert_eq!(super::part2(&(moves, boards)), 1924);
    }
}
