#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    X,
    O,
    None
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
            Player::None => write!(f, ""),
        }
    }
}

pub struct Board {
    pub board: [[Player; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[Player::None; 3]; 3],
        }
    }

    pub fn place(&mut self, cell_num: usize, player: Player) -> Result<(), String> {
        // Constraining cell_num's range between 1 to 9 exclusive
        if !(1..=9).contains(&cell_num) {return Err(String::from("cell_num should be between 1 to 10 inclusive"));}

        let mut cell_num = cell_num - 1;
        for (i, row) in self.board.clone().iter().enumerate() {
            let mut found = false;
            for (j, _) in row.iter().enumerate() {
                if cell_num == 0 {
                    if self.board[i][j] != Player::None {return Err(String::from("Cell is occupied"));}
                    self.board[i][j] = player;
                    found = true;
                    break;
                }
                cell_num -= 1;
            }
            if found {break;}
        }
        Ok(())
    }

    pub fn winner(&self) -> Player {
        if
            // HORIZONTAL
            self.board[0][0] == Player::X && self.board[0][1] == Player::X && self.board[0][2] == Player::X ||
            self.board[1][0] == Player::X && self.board[1][1] == Player::X && self.board[1][2] == Player::X ||
            self.board[2][0] == Player::X && self.board[2][1] == Player::X && self.board[2][2] == Player::X ||
            // VERTICAL
            self.board[0][0] == Player::X && self.board[1][0] == Player::X && self.board[2][0] == Player::X ||
            self.board[0][1] == Player::X && self.board[1][1] == Player::X && self.board[2][1] == Player::X ||
            self.board[0][2] == Player::X && self.board[1][2] == Player::X && self.board[2][2] == Player::X ||
            // DIAGONAL
            self.board[0][0] == Player::X && self.board[1][1] == Player::X && self.board[2][2] == Player::X ||
            self.board[0][2] == Player::X && self.board[1][1] == Player::X && self.board[2][0] == Player::X {
            Player::X
        } else if
            // HORIZONTAL
            self.board[0][0] == Player::O && self.board[0][1] == Player::O && self.board[0][2] == Player::O ||
            self.board[1][0] == Player::O && self.board[1][1] == Player::O && self.board[1][2] == Player::O ||
            self.board[2][0] == Player::O && self.board[2][1] == Player::O && self.board[2][2] == Player::O ||
            // VERTICAL
            self.board[0][0] == Player::O && self.board[1][0] == Player::O && self.board[2][0] == Player::O ||
            self.board[0][1] == Player::O && self.board[1][1] == Player::O && self.board[2][1] == Player::O ||
            self.board[0][2] == Player::O && self.board[1][2] == Player::O && self.board[2][2] == Player::O ||
            // DIAGONAL
            self.board[0][0] == Player::O && self.board[1][1] == Player::O && self.board[2][2] == Player::O ||
            self.board[0][2] == Player::O && self.board[1][1] == Player::O && self.board[2][0] == Player::O {
            Player::O
        } else {
            Player::None
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let mut cell_num = 0;
        for row in self.board.iter() {
            output.push_str("-------------\n|");
            for cell in row.iter() {
                if *cell == Player::None {
                    output.push_str(&format!(" {} |", cell_num + 1));
                } else {
                    output.push_str(&format!(" {} |", cell));
                }
                cell_num += 1;
            }
            output.push('\n'); 
        }
        output.push_str("-------------");
        write!(f, "{}", output)
    }
}
