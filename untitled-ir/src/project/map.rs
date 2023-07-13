use super::internal::MapInternals;
use rgb::RGB8;

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub internals: MapInternals,
    pub name: String,
    pub color: RGB8,
    pub size: (u16, u16),
}

impl Map {
    pub fn new(name: String) -> Self {
        Map {
            internals: MapInternals::new(),
            name,
            color: RGB8 {r:0, g:0, b:0},
            size: (1080, 720),
        }
    }
}