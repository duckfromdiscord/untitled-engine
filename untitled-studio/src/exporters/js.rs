use untitled_ir::project::*;
use untitled_js::game_html_script;

pub fn js_project(project: Project) -> (String, String) {
    return game_html_script(project)
}