mod internal;
pub mod map;

use serde::{Serialize, Deserialize};

use crate::project::map::Map;

#[derive(Clone, Serialize, Deserialize)]
pub enum FocusedObjectType {
    Project,
    Map,
    Object,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub maps: Vec<Map>,
    pub window_size: (usize, usize),
    pub selected_map_index: usize,
    pub focused_object_type: FocusedObjectType,
    pub focused_object_index: usize,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: "New project".to_string(),
            maps: vec![Map::new("Map 0".into())],
            window_size: (1080, 720),
            selected_map_index: 0,
            focused_object_type: FocusedObjectType::Project,
            focused_object_index: 0,
        }
    }
}