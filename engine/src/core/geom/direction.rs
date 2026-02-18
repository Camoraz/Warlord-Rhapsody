use super::delta::Delta;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    pub fn dir_vec(&self) -> Delta {
        match self {
            Self::Right     => Delta::new(1, 0),
            Self::DownRight => Delta::new(1, -1),
            Self::DownLeft  => Delta::new(0, -1),
            Self::Left      => Delta::new(-1, 0),
            Self::UpLeft    => Delta::new(-1, 1),
            Self::UpRight   => Delta::new(0, 1),
        }
    }

    pub const ALL: [Direction; 6] = [
        Self::Right,
        Self::DownRight,
        Self::DownLeft,
        Self::Left,
        Self::UpLeft,
        Self::UpRight,
    ];

    pub fn invert(&self) -> Direction {
        match self {
            Self::Right     => Self::Left,
            Self::Left      => Self::Right,
            Self::UpRight   => Self::DownLeft,
            Self::UpLeft    => Self::DownRight,
            Self::DownRight => Self::UpLeft,
            Self::DownLeft  => Self::UpRight,
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        Self::ALL.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_vec() {
        assert_eq!(Direction::Right.dir_vec(), Delta::new(1, 0));
        assert_eq!(Direction::UpLeft.dir_vec(), Delta::new(-1, 1));
        assert_eq!(Direction::DownLeft.dir_vec(), Delta::new(0, -1));
    }

    #[test]
    fn test_invert() {
        assert_eq!(Direction::Right.invert(), Direction::Left);
        assert_eq!(Direction::UpLeft.invert(), Direction::DownRight);
    }
}
