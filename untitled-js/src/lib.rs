use untitled_ir::project::Project;

const HTML: &str = include_str!("../resources/index.html");

pub fn game_script(project: Project) -> String {
    let mut script = include_str!("../resources/game.js").to_string();
    script = script.replace("%WIDTH%", &project.window_size.0.to_string());
    script = script.replace("%HEIGHT%", &project.window_size.1.to_string());
    script += "\r\nfunction preload() {\r\n";
    script += "};\r\n";
    script += "\r\nfunction create() {\r\n";
    script += "};\r\n";
    script += "\r\nfunction update() {\r\n";
    script += "};\r\n";
    return script;
}

pub fn game_html_script(project: Project) -> (String, String) {
    return (HTML.to_string(), game_script(project));
}