//use crate::utils::challenge::Challenge;
use std::fs;
use std::collections::HashMap;
use std::thread;

pub struct Day4 {
    data: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct BingoBoard {
    data: HashMap<u32, (usize, usize)>,
    row_cnt: Vec<u32>,
    col_cnt: Vec<u32>,
}

impl BingoBoard {
    fn check(&mut self, num: &u32) -> bool {
        if self.data.contains_key(num) {
            let (row, col) = self.data.remove(num).unwrap();
            self.row_cnt[row] += 1;
            self.col_cnt[col] += 1;
        }

        self.row_cnt.iter().any(|&x| x > 4) || self.col_cnt.iter().any(|&x| x > 4)
    }

    fn play(&mut self, input: &[u32]) -> Option<(usize, u32, u32)> {
        for (i, v) in input.iter().enumerate() {
            if self.data.contains_key(v) && self.check(v) {
                let sum_unmarked: u32 = self.data.iter().map(|(key, _)| *key).sum();
                return Some((i, *v, sum_unmarked));
            }
        }

        None
    }
}

impl From<&[String]> for BingoBoard {
    fn from(input: &[String]) -> Self {
        let mut data: HashMap<u32, (usize, usize)> = HashMap::new();
        for (i, l) in input.iter().enumerate() {
            data.extend(
                l.split(' ')
                    .filter(|s| !s.is_empty())
                    .enumerate()
                    .map(|(j, s)| (s.parse::<u32>().unwrap(), (i, j)))
                    .collect::<HashMap<u32, (usize, usize)>>(),
            )
        }

        BingoBoard {
            data,
            row_cnt: vec![0, 0, 0, 0, 0],
            col_cnt: vec![0, 0, 0, 0, 0],
        }
    }
}

#[derive(Debug, PartialEq)]
struct BingoSubsystem {
    input: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl From<&[String]> for BingoSubsystem {
    fn from(data: &[String]) -> Self {
        let input: Vec<u32> = data[0]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let boards: Vec<BingoBoard> = (2..data.len())
            .step_by(6)
            .map(|i| BingoBoard::from(&data[i..i + 5]))
            .collect();

        BingoSubsystem { input, boards }
    }
}

impl BingoSubsystem {
    fn play(mut self) -> (u32, u32) {
        let handles: Vec<_> = (0..self.boards.len())
            .map(|_| {
                let mut board = self.boards.pop().unwrap();
                let input = self.input.clone();

                thread::spawn(move || board.play(&input))
            })
            .collect();

        let mut winner = self.input.len();
        let mut loser = 0;

        let mut winner_result = 0;
        let mut loser_result = 0;

        for h in handles {
            if let Some((idx, last, sum_unmarked)) = h.join().unwrap() {
                if idx < winner {
                    winner = idx;
                    winner_result = last * sum_unmarked;
                }
                if idx > loser {
                    loser = idx;
                    loser_result = last * sum_unmarked;
                }
            }
        }

        (winner_result, loser_result)
    }
}

// impl Challenge for Day4 {
//     fn new(input_file: &str) -> Self {
//         Self {
//             data: Self::load(input_file).unwrap(),
//         }
//     }

//     fn run(&self, part: u32) -> Result<String, String> {
//         match part {
//             1 => self.run_part_one(),
//             2 => self.run_part_two(),
//             x => unimplemented!(
//                 "Invalid part {} for Day {}",
//                 x,
//                 std::any::type_name::<Self>()
//             ),
//         }
//     }
// }

impl Day4 {
    fn run_part_one(&self) -> Result<String, String> {
        let bingo = BingoSubsystem::from(&self.data[..]);
        let (result, _) = bingo.play();
        Ok(format!("{:#?}", result))
    }

    fn run_part_two(&self) -> Result<String, String> {
        let bingo = BingoSubsystem::from(&self.data[..]);
        let (_, result) = bingo.play();
        Ok(format!("{:#?}", result))
    }
}

fn main() {
    let file = fs::read_to_string("day4in.txt").unwrap();
    let mut input: Vec<String> = file.lines().map(String::from).collect();
    let day4 = Day4 { data: input };
    println!("{:?}", day4.run_part_one());
    println!("{:?}", day4.run_part_two());
}