#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None
        }

        Some(Self {
            rank,
            file
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            position,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank
           || self.position.file == other.position.file {
            return true;
        }

        let dfile = (self.position.file - other.position.file).abs();
        let drank = (self.position.rank - other.position.rank).abs();

        if dfile == drank {
            return true;
        }

        false
    }
}
