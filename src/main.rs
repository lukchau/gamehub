use std::io;
use rand::Rng;

fn main() {
    println!("Welcome! What game do you want to play?");
    println!("1. Tic Tac Toe");
    println!("2. Rock Paper Scissors");

    let mut game_choice = String::new();
    io::stdin()
        .read_line(&mut game_choice)
        .expect("Failed to read input.");

    let game_choice: u32 = game_choice.trim().parse().expect("Invalid input");

    match game_choice {
        1 => play_tic_tac_toe(),
        2 => play_rock_paper_scissors(),
        _ => println!("Invalid choice, try again."),
    }
}

fn play_tic_tac_toe() {
    let mut board = vec![
        vec!['0', '1', '2'],
        vec!['3', '4', '5'],
        vec!['6', '7', '8'],
    ];

    print_board(&board);

    loop {
        let mut player_row = String::new();
        let mut player_column = String::new();

        println!("Enter the row number (0-2):");
        io::stdin()
            .read_line(&mut player_row)
            .expect("Failed to read input.");

        let player_row: usize = match player_row.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        };

        println!("Enter the column number (0-2):");
        io::stdin()
            .read_line(&mut player_column)
            .expect("Failed to read input.");

        let player_column: usize = match player_column.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        };

        if player_row >= 3 || player_column >= 3 {
            println!("Invalid move. Try again.");
            continue;
        }

        if board[player_row][player_column] != 'X' && board[player_row][player_column] != 'o' {
            board[player_row][player_column] = 'X';
            println!("Player o chose row {} and column {}", player_row, player_column);
            print_board(&board);
        } else {
            println!("Square already occupied. Try again.");
            continue;
        }

        if check_win(&board, 'X') {
            println!("Player X has won!");
            break;
        } else if check_tie(&board) {
            println!("It's a tie!");
            break;
        }

        let ai_move = ai_choose_move(&board);

        board[ai_move.0][ai_move.1] = 'o';
        println!("AI chose row {} and column {}", ai_move.0, ai_move.1);
        print_board(&board);

        if check_win(&board, 'o') {
            println!("Player o has won!");
            break;
        } else if check_tie(&board) {
            println!("It's a tie!");
            break;
        }
    }
}

fn play_rock_paper_scissors() {
    println!("Choose one of three options:");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissors");

    loop {
        let mut player_choice = String::new();
        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read player's choice.");

        let player_choice: u32 = match player_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please choose a number between 1 and 3.");
                continue;
            }
        };

        if !(1..=3).contains(&player_choice) {
            println!("Invalid input. Please choose a number between 1 and 3.");
            continue;
        }

        let ai_choice = rand::thread_rng().gen_range(1..=3);

        let player_choice_str = match player_choice {
            1 => "Rock",
            2 => "Paper",
            3 => "Scissors",
            _ => unreachable!(),
        };

        let ai_choice_str = match ai_choice {
            1 => "Rock",
            2 => "Paper",
            3 => "Scissors",
            _ => unreachable!(),
        };

        println!("You chose: {}", player_choice_str);
        println!("AI chose: {}", ai_choice_str);

        match player_choice {
            1 => {
                if ai_choice == 3 {
                    println!("You win!");
                } else if ai_choice == 2 {
                    println!("AI wins!");
                } else {
                    println!("It's a tie!");
                }
            }
            2 => {
                if ai_choice == 1 {
                    println!("You win!");
                } else if ai_choice == 3 {
                    println!("AI wins!");
                } else {
                    println!("It's a tie!");
                }
            }
            3 => {
                if ai_choice == 2 {
                    println!("You win!");
                } else if ai_choice == 1 {
                    println!("AI wins!");
                } else {
                    println!("It's a tie!");
                }
            }
            _ => unreachable!(),
        }

        break;
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        println!("{} | {} | {}", row[0], row[1], row[2]);
        println!("---------");
    }
}

fn check_win(board: &Vec<Vec<char>>, player: char) -> bool {
    let winning_combos: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combo in &winning_combos {
    let mut positions = combo.iter().map(|&x| (x / 3, x % 3));
    if positions.all(|(row, col)| board[row][col] == player) {
        return true;
    }
}

    false
}

fn check_tie(board: &Vec<Vec<char>>) -> bool {
    board.iter().all(|row| row.iter().all(|&square| square != 'X' && square != 'o'))
}

fn ai_choose_move(board: &Vec<Vec<char>>) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let mut row;
    let mut column;

    loop {
        row = rng.gen_range(0..3);
        column = rng.gen_range(0..3);

        if board[row][column] != 'X' && board[row][column] != 'o' {
            break;
        }
    }

    (row, column)
}