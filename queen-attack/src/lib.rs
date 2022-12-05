#[derive(Debug)]
pub struct ChessPosition
{
    row: i32,
    col: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file)
        {
            Some(Self {
                row: rank,
                col: file,
            })
        } else 
        {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            pos: position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let row_diff = (self.pos.row - other.pos.row).abs();
        let col_diff = (self.pos.col - other.pos.col).abs();
        if self.pos.row == other.pos.row {
            true
        } else if self.pos.col == other.pos.col {
            true
        } else {
            row_diff == col_diff
        }
    }
}
