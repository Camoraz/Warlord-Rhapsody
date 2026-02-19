#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delta {
    dx: i16,
    dy: i16,
}

impl Delta {
    pub const fn new(dx: i16, dy: i16) -> Self {
        Self { dx, dy }
    }

    pub const fn dx(&self) -> i16 { self.dx }
    pub const fn dy(&self) -> i16 { self.dy }

    pub fn invert(&self) -> Delta {
        Self::new(
            -self.dx,
            -self.dy,
        )
    }

    pub fn norm(&self) -> i16 {
        let s = -self.dx - self.dy;

        (self.dx.abs() + self.dy.abs() + s.abs()) / 2
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert() {
        let d = Delta::new(3, -4);
        let inv = d.invert();
        assert_eq!(inv.dx, -3);
        assert_eq!(inv.dy, 4);
    }

    #[test]
    fn test_norm() {
        assert_eq!(Delta::new(2, 1).norm(), 3);
        assert_eq!(Delta::new(-1, 1).norm(), 1);
        assert_eq!(Delta::new(-2, 1).norm(), 2);
        assert_eq!(Delta::new(-2, -2).norm(), 4);
    }
}
