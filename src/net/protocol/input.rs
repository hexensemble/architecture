use bitcode::{Decode, Encode};

#[derive(Clone, Copy, Debug, Decode, Encode, PartialEq)]
pub enum PlayerInput {
    Up,
    Down,
    Left,
    Right,
}
