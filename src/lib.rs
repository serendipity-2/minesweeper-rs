use rand::Rng;

#[derive(Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct MSBoard {
    pub width: u8,
    pub length: u8,
    pub mines: Vec<Position>,
    pub flags: Vec<Position>,
    pub cleared: Vec<Position>,
}

pub enum GameState {
    StillPlaying,
    Lost,
    Won,
}

impl MSBoard {
    pub fn new(width: u8, length: u8, nmines: u8) -> MSBoard {

        let mut mines: Vec<Position> = Vec::<Position>::new();
        let flags: Vec<Position> = Vec::<Position>::new();
        let cleared: Vec<Position> = Vec::<Position>::new();
        
        let mut mines_added: u8 = 0;
        while mines_added < nmines {
            
            let x = rand::thread_rng().gen_range(0..width);
            let y = rand::thread_rng().gen_range(0..length);
            let pos: Position = Position { x, y, };
            if !mines.contains(&pos) {
                mines.push(pos);
                mines_added += 1;
            }

        }

        MSBoard {
            width, length, flags, cleared, mines,
        }
    }

    // fn mine_at(&self, pos: &Position) -> bool { self.mines.contains(pos) }

    // send false on loss
    pub fn send_move(&mut self, pos: Position) -> GameState {
        if self.cleared.contains(&pos) || self.flags.contains(&pos) {
            GameState::StillPlaying  // nothing happens when clicking on cleared or flag tile
        }
        else if self.mines.contains(&pos) {
            GameState::Lost // clicked on a mine
        }
        else {
            self.cleared.push(pos);
            GameState::StillPlaying
        }
        // TODO
        // if check_for_win() { Won }
    }

    pub fn show_stuff(&self) {
        for mine in &self.mines[..] {
            println!("Mine at ({:?},{:?})", mine.x, mine.y);
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_amount_of_mines() {
        let board: MSBoard = MSBoard::new(20, 20, 40);
        assert_eq!(40, board.mines.len());
    }

//    #[test]
//    fn test_send_move() {
//        let board: 
//    }

}
