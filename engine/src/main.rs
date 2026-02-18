mod core;
mod render;

use crate::core::geom;
use crate::render::cli::render_deltas;

fn main(){
    let mut disk = geom::shapes::ring(5);
    let mut disk2 = geom::shapes::disk(3);
    let mut disk1 = geom::shapes::ring(9);

    disk.append(&mut disk2);
    disk.append(&mut disk1);

    render_deltas(&disk);
}