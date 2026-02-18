use super::delta::Delta;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y } }

    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }

    pub fn offset(&self, delta: Delta) -> Self {
        Self {
            x: ((self.x as isize) + (delta.dx() as isize)) as usize,
            y: ((self.y as isize) + (delta.dy() as isize)) as usize,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::core::geom::Delta;
    use super::*;

    #[test]
    fn test_offset() {
        let pos = Position::new(5, 5);
        let delta = Delta::new(2, -3);
        let new_pos = pos.offset(delta);
        assert_eq!(new_pos.x, 7);
        assert_eq!(new_pos.y, 2);
    }
}
