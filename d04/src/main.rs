use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() {
    println!("Hello, world!");
}

type Board = Vec<Vec<u32>>;

type Coords = (usize, usize);

#[derive(Debug)]
struct Bingo {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

fn parse(path: &str) -> Result<Bingo, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;

    let mut rows = input.trim_end().split("\n");
    let draws = rows.next().ok_or("no draw row found")?;
    let draws = draws
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    rows.next();

    let mut boards = vec![];
    let mut board = vec![];

    for row in rows {
        if row == "" {
            boards.push(board);
            board = vec![];
        } else {
            let row = row
                .split_whitespace()
                .map(|x| x.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()?;

            board.push(row);
        }
    }

    boards.push(board);

    Ok(Bingo { draws, boards })
}

// fn transpose(board: &Board) -> Board {
//     let mut transposed = board.clone();

//     for row_idx in 0..board.len() {
//         for col_idx in 0..board[0].len() {
//             transposed[col_idx][row_idx] = board[row_idx][col_idx];
//         }
//     }

//     transposed
// }

type MarkedSet = HashSet<(usize, usize)>;
type Draw = u32;
type BoardId = usize;
type Win = (Draw, BoardId, MarkedSet);

fn find_winner(bingo: &Bingo) -> Vec<Win> {
    let mut board_to_matches: HashMap<usize, Vec<Coords>> = HashMap::new();
    let mut known_winners = HashSet::new();
    let mut wins: Vec<Win> = vec![];

    for draw in &bingo.draws {
        // mark matches
        for (board_id, board) in bingo.boards.iter().enumerate() {
            for (row_idx, row) in board.iter().enumerate() {
                for (cell_idx, cell) in row.iter().enumerate() {
                    if cell == draw {
                        let matches = board_to_matches.entry(board_id).or_default();
                        matches.push((row_idx, cell_idx));
                    }
                }
            }
        }

        // check if there's a board with 5 marked numbers on the same row or column
        for (board_id, matches) in &board_to_matches {
            if !known_winners.contains(board_id) {
                let mut row_counts: HashMap<usize, u32> = HashMap::new();
                let mut col_counts: HashMap<usize, u32> = HashMap::new();

                for (row_idx, col_idx) in matches {
                    let row_count = row_counts.entry(*row_idx).or_default();
                    *row_count += 1;

                    let col_count = col_counts.entry(*col_idx).or_default();
                    *col_count += 1;
                }

                if row_counts.values().any(|count| *count == 5)
                    || col_counts.values().any(|count| *count == 5)
                {
                    let win = (*draw, *board_id, HashSet::from_iter(matches.clone()));

                    wins.push(win);
                    known_winners.insert(*board_id);
                }
            }
        }
    }

    wins
}

fn answer(bingo: &Bingo, winner: &Win) -> u32 {
    let (draw, board_id, matches) = winner;

    let board = &bingo.boards[*board_id];
    let mut sum_unmarked = 0;
    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if !matches.contains(&(row_idx, col_idx)) {
                sum_unmarked += cell;
            }
        }
    }

    sum_unmarked * draw
}

fn p1(bingo: &Bingo) -> Result<u32, Box<dyn Error>> {
    let winner = &find_winner(bingo)[0];
    Ok(answer(bingo, winner))
}

fn p2(bingo: &Bingo) -> Result<u32, Box<dyn Error>> {
    let winners = find_winner(bingo);
    let winner = &winners[winners.len() - 1];
    Ok(answer(bingo, winner))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_test() {
        let bingo = parse("d04_test_input").unwrap();
        let p1_ans = p1(&bingo).unwrap();
        assert_eq!(p1_ans, 4512);

        let bingo = parse("d04_input").unwrap();
        let p1_ans = p1(&bingo).unwrap();
        assert_eq!(p1_ans, 8136);
    }

    #[test]
    fn p2_test() {
        let bingo = parse("d04_test_input").unwrap();
        let p2_ans = p2(&bingo).unwrap();
        assert_eq!(p2_ans, 1924);

        let bingo = parse("d04_input").unwrap();
        let p2_ans = p2(&bingo).unwrap();
        assert_eq!(p2_ans, 12738);
    }
}
