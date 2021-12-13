use std::fs;

struct BoardSlot {
    checked: bool,
    value: i32,
}

impl std::fmt::Debug for BoardSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", if self.checked { "x" } else { " " }, self.value)
    }
}

struct Board {
    slots: Vec<BoardSlot>
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let row0: Vec<_> = self.slots.iter().take(5).collect();
        let row1: Vec<_> = self.slots.iter().skip(5).take(5).collect();
        let row2: Vec<_> = self.slots.iter().skip(10).take(5).collect();
        let row3: Vec<_> = self.slots.iter().skip(15).take(5).collect();
        let row4: Vec<_> = self.slots.iter().skip(20).take(5).collect();

        for row in [row0, row1, row2, row3, row4] {
            
            write!(f, "{:?} {:?} {:?} {:?} {:?}\n", row[0], row[1], row[2], row[3], row[4] );
        }

        Ok(())
    }
}

impl Board {
    fn new() -> Board {
        Board { slots: Vec::<BoardSlot>::new() }
    }

    fn check_off(&mut self, new_value: i32) {
        for slot in self.slots.iter_mut() {
            if slot.value == new_value {
                slot.checked = true;
            }
        }
    }

    fn has_won(&self) -> bool {
        let index_groups = [
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
            // [0, 6, 12, 18, 24], // WHAT THE FUCK DIAGONALS DON'T COUNT?????????
            // [4, 8, 12, 16, 20]  // FUCK YOUUUUUUUUUUUUUU!!!!
        ];

        for group in index_groups {
            let mut checks = 0;
            for index in group {
                if self.slots[index].checked == false { break; }
                checks += 1;
            }

            if checks == 5 { return true; }
        }

        return false;
    }

    fn sum_of_unchecked(&self) -> i32 {
        self.slots.iter()
            .filter(|&slot| slot.checked == false)
            .fold(0, |sum, slot| sum + slot.value)
    }
}

fn main() {
    let file = fs::read_to_string("day4in.txt").unwrap();
    let mut input = file.lines();

    let sequence_str = input.next().unwrap();

    let sequence: Vec<i32> = sequence_str
        .split(",")            
        .map(|s| s.parse().unwrap())
        .collect();

    // println!("{:?}", sequence);

    let mut current_board = Board::new();
    let mut boards = Vec::<Board>::new();

    // drop empty line becuase i don't feel like checking for it
    input.next();

    for line in input {
        if line.is_empty() {
            boards.push(current_board);
            current_board = Board::new();

            continue;
        }

        let line_numbers: Vec<i32> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut new_slots: Vec<BoardSlot> = line_numbers.iter().map(|value| BoardSlot { value: *value, checked: false}).collect();
        current_board.slots.append(&mut new_slots);
    }

    boards.push(current_board);

    let board_count = boards.len();
    let mut boards_that_have_won = vec![false; board_count];
    let mut winning_board_count = 0;

    for drawn in sequence {
        println!("== DRAWING {} ==", drawn);

        let mut i = 0;
        for board in boards.iter_mut() {
            if boards_that_have_won[i] { i += 1; continue }
            
            board.check_off(drawn);
            if board.has_won() {
                boards_that_have_won[i] = true;
                winning_board_count += 1;
                println!("Board {} won", i);
                println!("{:?}", board);
                let sum = board.sum_of_unchecked();
                println!("score: {} board {}", sum * drawn, i);
                println!("Boards left: {}", board_count - winning_board_count);
                if board_count == winning_board_count { break; }
            }
            i += 1;    
        }
        if board_count == winning_board_count { break; }
    }
    // println!("ALL BOARDS:");
    // println!("{:?}", boards);
}