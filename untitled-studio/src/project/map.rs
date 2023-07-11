use super::internal::MapInternals;
use rgb::RGB8;

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub internals: MapInternals,
    pub name: String,
    pub color: RGB8,
}

impl Map {
    pub fn new(name: String) -> Self {
        Map {
            internals: MapInternals::new(),
            name,
            color: RGB8 {r:255, g:255, b:255},
        }
    }
}