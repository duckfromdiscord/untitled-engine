use untitled_ir::project::Project;

pub fn game_scripts(project: Project) -> Vec<(String, Option<String>)> {
    let mut scripts = vec![];

    let mut main = include_str!("../resources/main.rs.txt").to_string();
    main = main.replace("%R%", &0.to_string());
    main = main.replace("%G%", &0.to_string());
    main = main.replace("%B%", &0.to_string());

    main = main.replace("%WIDTH%", &project.window_size.0.to_string());
    main = main.replace("%HEIGHT%", &project.window_size.1.to_string());
    let safe_name = project.name.replace("\"", "\\\"");
    main = main.replace("%TITLE%", &safe_name);

    scripts.push(("src".to_string(), None));
    scripts.push(("src/main.rs".to_string(), Some(main)));

    let toml = include_str!("../resources/Cargo.toml.txt").to_string();
    scripts.push(("Cargo.toml".to_string(), Some(toml)));

    return scripts;
}
