extern crate colorguess;

use colorguess::{Board, BoardRow, Pegs, build_all_configs, count_outcomes, get_score, get_random_pegs, print_score_histogram};

fn strategy_greedy(board: &Board) -> Pegs {
    if board.possible.len()==1 {
        return board.possible[0].clone();
    }
    let all = build_all_configs();
    all.iter()
    .min_by_key(|g| count_outcomes(g, &board.possible).iter().max().unwrap().clone())
    .unwrap().clone()
}

// TODO: https://crates.io/crates/criterion

fn main() {
    let all = build_all_configs();
    println!("1234: {:?}", count_outcomes(&[1,2,3,4], &all));
    // println!("1233: {:?}", count_outcomes(&[1,2,3,3], &all));
    // println!("1122: {:?}", count_outcomes(&[1,1,2,2], &all));

    let secret = get_random_pegs();
    let strategy = strategy_greedy;
    let mut board = Board::new();
    println!("Starting");
    loop {
        if board.is_complete() {
            break;
        }
        let guess = strategy(&board);
        let chances = count_outcomes(&guess, &board.possible);
        println!("Outcome distribution for selected guess:");
        print_score_histogram(&chances);

        let score = get_score(&secret, &guess);
        let row = BoardRow {guess, score};
        print!("Tried row: {:}. ", &row);
        board.add_guess(row);
        println!("... {} possibilities left", board.possible.len());
    }
    //println!("{:}", board);
    println!("Secret was: {:?}", secret);
}