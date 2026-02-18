use crate::core::geom::delta::{Delta};
use std::cmp::{min, max};

pub fn render_deltas(deltas: &[Delta]) {
    use std::cmp::{min, max};

    if deltas.is_empty() {
        println!("No deltas to render.");
        return;
    }

    // Map axial (dx, dy) to screen coordinates
    let screen_coords: Vec<(i16, i16)> = deltas
        .iter()
        .map(|d| (d.dx() + d.dy(), d.dx()))
        .collect();

    // Determine bounds
    let min_col = screen_coords.iter().map(|(c, _)| *c).min().unwrap();
    let max_col = screen_coords.iter().map(|(c, _)| *c).max().unwrap();
    let min_row = screen_coords.iter().map(|(_, r)| *r).min().unwrap();
    let max_row = screen_coords.iter().map(|(_, r)| *r).max().unwrap();

    for row in (min_row..=max_row).rev() {
        // print spaces to stagger the row (row number spaces)
        let indent = max_row - row;

        for i in 0..indent {
            let is_dot = (i + indent) % 2 == 0;
            print!("{}", if is_dot { '·' } else { ' ' });
        }

        for col in min_col..=max_col {
            if screen_coords.iter().any(|(c, r)| *c == col && *r == row) {
                print!("@ ");
            } else {
                print!("· ");
            }
        }

        for i in 0..row {
            let is_dot = i % 2 == 0;
            print!("{}", if is_dot { '·' } else { ' ' });
        }

        println!();
    }
}
