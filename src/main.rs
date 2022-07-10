use minesweeper::MSBoard;
use minesweeper::Position;
use minesweeper::GameState;

use rand::Rng;

fn main() {

    let width: u8 = 20;
    let height: u8 = 20;

    let mut board: MSBoard = MSBoard::new(width, height, 40);
    
    loop {

        let x: u8 = rand::thread_rng().gen_range(0..width); 
        let y: u8 = rand::thread_rng().gen_range(0..height); 

        let state: GameState = board.send_move(Position { x,y, });
        println!("Sending move - ({},{})", x, y);
        match state { 
            GameState::Lost => {
                println!("You lose!");
                break;
            },
            GameState::Won => {
                println!("You win!");
                break;
            },
            GameState::StillPlaying => {
                continue;
            }
        }
    }
}
