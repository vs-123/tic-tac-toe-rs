use std::io;
use std::io::Write;
use rand::Rng;

mod game;
use game::{Board, Player};

fn main() {
    println!("TIC TAC TOE\n");
    println!("How to play:");
    println!("Enter the move based on the number of the cell on the board\n\n");
    let mut round = 1;

    loop {
        println!("Round {}\n{}\n\n", round, "-".repeat(format!("Round {}", round).len()));
        let mut game: Board = Board::new();
        let mut available_cells = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut rng = rand::thread_rng();
        while game.winner() == Player::None || available_cells.is_empty() {
            println!("{}", game);
            let mut player_move = String::new();
            print!("Enter your move: ");
            io::stdout().flush().expect("Could not flush stdout");
            if io::stdin().read_line(&mut player_move).is_err() {
                continue;
            }
            if let Ok(player_move) = player_move.trim_end().parse::<usize>() {
                game.place(player_move, Player::X).expect("Invalid move");
                available_cells = available_cells.iter().filter(|&cell| *cell != player_move).copied().collect();
            } else {
                continue;
            }
            // Checking if it's a tie
            if available_cells.is_empty() && game.winner() == Player::None {
                break;
            }
            // Opponent's move
            let mut opponent_move = rng.gen_range(0..available_cells.len());
            opponent_move = available_cells[opponent_move];
            game.place(opponent_move, Player::O).expect("Invalid move");
            available_cells = available_cells.iter().filter(|&cell| *cell != opponent_move).copied().collect();
            println!();
        }
        // Showing the final board before result
        println!("\n{}", game);
        // Not checking for Player::None as it has been already checked in the loop
        if available_cells.is_empty() { 
            println!("It's a tie!");
            continue;
        }
        println!("{} won the match!", game.winner());
        round += 1;
    }
}

#[test]
fn tictactoe_test() {
    let mut game = Board::new();
    game.place(1, Player::X);
    game.place(3, Player::O);
    game.place(5, Player::O);
    game.place(7, Player::O);
    game.place(2, Player::X);
    game.place(9, Player::X);
    println!("{}", game.to_string());
    assert_eq!(game.winner(), Player::O);
}
