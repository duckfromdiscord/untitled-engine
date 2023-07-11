use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MapInternals {
    pub name_selected: bool,
}

impl MapInternals {
    pub fn new() -> Self {
        MapInternals {
            name_selected: false,
        }
    }
}