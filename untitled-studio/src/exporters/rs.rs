use untitled_ir::project::*;
use untitled_rs::game_scripts;

pub fn rs_project(project: Project) -> Vec<(String, Option<String>)> {
    return game_scripts(project)
}