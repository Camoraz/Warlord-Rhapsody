use crate::core::geom::direction;

use super::delta::Delta;
use super::direction::Direction;

/// All 6 hex directions
pub fn neighbors() -> Vec<Delta> {
    vec![
        Direction::Right.dir_vec(),
        Direction::Left.dir_vec(),
        Direction::UpRight.dir_vec(),
        Direction::UpLeft.dir_vec(),
        Direction::DownRight.dir_vec(),
        Direction::DownLeft.dir_vec(),
    ]
}

/// Returns all hex offsets within a given radius (filled disk)
/// Includes center (0,0)
pub fn disk(radius: i16) -> Vec<Delta> {
    let mut results = Vec::new();

    for q in -radius..=radius {
        // Since in cubic coords, a disk is |x|, |y|, |z| <= R,
        // and in axial, z = y - x, we enforce the z constraint here.
        let r1 = (-radius).max(-q - radius);
        let r2 = (radius).min(-q + radius);

        for r in r1..=r2 {
            results.push(Delta::new(q, r));
        }
    }

    results
}

pub fn ring(radius: i16) -> Vec<Delta> {
    if radius == 0 {
        return vec![Delta::new(0, 0)];
    }

    let directions = super::direction::Direction::iter().map(|d| d.dir_vec());;

    let mut results = Vec::with_capacity((radius * 6) as usize);

    // Start at one corner of the ring
    let mut q = -radius;
    let mut r = radius;

    // Walk the 6 sides
    for d in directions {
        for _ in 0..radius {
            results.push(Delta::new(q, r));
            q += d.dx();
            r += d.dy();
        }
    }

    results
}

pub fn cone(length: i16) -> Vec<Delta> {
    return vec![]
}

