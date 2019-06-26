#[derive(Debug)]
pub struct ChessPosition (i32,i32);

#[derive(Debug)]
pub struct Queen (ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (x, y) if x < 0 || y < 0 => None,
            (x, y) if x >= 8 || y >= 8 => None,
            (x, y) => Some(ChessPosition(x, y))
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match (&self, other) {
            (x, y) if (x.0).0 == (y.0).0 || (x.0).1 == (y.0).1 => true,
            (x, y) if (x.0).0 == (x.0).1 && (y.0).0 == (y.0).1 => true,
            _ => false
        }
    }
}
