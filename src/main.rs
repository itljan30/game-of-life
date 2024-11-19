use rand::Rng;
use std::{thread, time::Duration, io::{self, Write}};

fn draw_frame(board: &Vec<Vec<i32>>) {
    let mut screen = String::new();
    for row in board {
        for &cell in row {
            if cell == 0 {
                screen.push(' ');
                screen.push(' ');
            }
            else if cell == 1 {
                screen.push('█');
                screen.push('█');
            }
        }
        screen.push('\n');
    }
    print!("\x1B[H");

    // print!("{}", screen);
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", screen).unwrap();
}

fn randomize_board(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let r = if rng.gen_bool(0.5) {1} else {0};
            board[i][j] = r;
        }
    }
    board
}

// fewer than 2 neighbors dies
// 2 or 3 neighbors lives
// more than 3 dies
// dead cell with 3 neighbors comes to life
fn update_board(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_board = vec![vec![0; board[0].len()]; board.len()];
    for i in 0..board.len() as isize {
        for j in 0..board[i as usize].len() as isize {
            // count neighbors
            let mut neighbors: i32 = 0;
            for k in i-1..=i+1 as isize {
                for l in j-1..=j+1 as isize {
                    if k < 0 || k as usize >= board.len(){
                        continue;
                    } 
                    if l < 0 || l as usize >= board[k as usize].len() {
                        continue;
                    }
                    if k == i && j == l {
                        continue;
                    }
                    if board[k as usize][l as usize] == 1 {
                        neighbors += 1;
                    }
                }
            }
            // apply logic
            if board[i as usize][j as usize] == 1 && neighbors < 2 {
                new_board[i as usize][j as usize] = 0;
            }
            else if board[i as usize][j as usize] == 1 && neighbors > 3 {
                new_board[i as usize][j as usize] = 0;
            }
            else if board[i as usize][j as usize] == 0 && neighbors == 3 {
                new_board[i as usize][j as usize] = 1;
            }
            else if board[i as usize][j as usize] == 1 && 2 <= neighbors && neighbors <= 3 {
                new_board[i as usize][j as usize] = 1;
            }
        }
    }
    return new_board;
}

fn main() {
    println!("Fullscreen for the best experience.\nYou might need to zoom out (usually 'Ctrl + -').");
    println!("Use 'Ctr + C' to stop the simulation.\nPress ENTER to start the simulation.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // let mut board: Vec<Vec<i32>> = vec![vec![0; 40]; 24]; // default terminal size
    let mut board: Vec<Vec<i32>> = vec![vec![0; 95]; 50]; // default terminal size fullscreened

    // let mut board: Vec<Vec<i32>> = vec![vec![0; 190]; 97]; // fullscreened and zoome out a bit
    // let mut board: Vec<Vec<i32>> = vec![vec![0; 950]; 540]; // most zoomed out

    board = randomize_board(board);

    loop {
        draw_frame(&board);
        board = update_board(board);
        thread::sleep(Duration::from_millis(50))
    }
}
