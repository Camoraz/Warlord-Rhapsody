#[derive(Clone, Copy)]
pub struct PlayerId(u32);
impl PlayerId { pub fn new(val: u32) -> Self { PlayerId(val) }} 

#[derive(Clone)]
pub struct Player {
    name: String,
}
