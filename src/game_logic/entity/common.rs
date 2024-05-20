pub use super::super::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(hp: u32) -> Self {
        Health {
            current: hp,
            max: hp,
        }
    }
}

pub struct FieldOfView
{
    pub shape: Box<dyn Shape>,
}
