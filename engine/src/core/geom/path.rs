use super::direction::Direction;
use super::position::Position;

pub struct Path {
    path: Vec<Direction>,
    start: Position,
    end: Position,
}

impl Path {
    pub fn new(path: Vec<Direction>, start: Position, end: Position) -> Self {
        Self {
            path : path,
            start: start,
            end: end,
        }
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = &'a Direction;
    type IntoIter = std::slice::Iter<'a, Direction>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.iter()
    }
}
