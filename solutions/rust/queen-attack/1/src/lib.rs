use std::ops::Range;

#[derive(Debug)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

const VALID_NUMBERS: Range<i32> = 0..8;
impl ChessPosition {
    #[must_use]
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if VALID_NUMBERS.contains(&rank) && VALID_NUMBERS.contains(&file) {
            Some(Self {
                rank: rank as u8,
                file: file as u8,
            })
        } else {
            None
        }
    }
}

impl Queen {
    #[must_use]
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    #[must_use]
    fn x(&self) -> u8 {
        self.position.file
    }
    #[must_use]
    fn y(&self) -> u8 {
        self.position.rank
    }

    #[must_use]
    pub fn can_attack(&self, other: &Queen) -> bool {
        self.x() == other.x()
            || self.y() == other.y()
            || self.x() + self.y() == other.x() + other.y()
            || self.x() as i8 - self.y() as i8 == other.x() as i8 - other.y() as i8
    }
}
