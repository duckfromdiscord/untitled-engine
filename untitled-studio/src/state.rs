use untitled_ir::project::Project;
use egui_dock::Tree;

pub struct UntitledStudioApp {
    // Main menu
    pub main_menu: bool,
    pub project: Project,
    pub project_path: Option<String>,
    // For a loaded project
    pub tree: Tree<String>,
    pub status: String,
    pub modified: bool,
}