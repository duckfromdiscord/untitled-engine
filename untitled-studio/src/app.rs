use untitled_ir::project::Project;
use crate::ui::strings::*;

use egui_dock::{DockArea, NodeIndex, Style, Tree};

use std::{io::Write, fs::File};

pub use crate::state::*;
pub use crate::ui::tabs::*;

impl Default for UntitledStudioApp {
    fn default() -> Self {
        let mut tree = Tree::new(vec!["Map View".to_owned()]);
        
        let [_, b] = tree.split_left(NodeIndex::root(), 0.14, vec!["Project Tree".to_owned()]);
        let [_, _] = tree.split_below(b, 0.5, vec!["Object View".to_owned()]);

        UntitledStudioApp {
            main_menu: true,
            project: Project::default(),
            project_path: None,
            tree,
            status: "Ok.".to_string(),
            modified: true,
        }
    }
}

impl UntitledStudioApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts);
        cc.egui_ctx.set_fonts(fonts);

        Default::default()
    }
}

impl eframe::App for UntitledStudioApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.main_menu {
            // Let the user choose a project or create a new one
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("Welcome to Untitled Studio!")
                        .italics()
                        .weak()
                        .size(16.0),
                );

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(200.0); //spacing from top of the screen to top of first button
                    if ui
                        .button(egui::RichText::new(format!(
                                "{} Create a new project.",
                                egui_phosphor::FOLDER_PLUS))
                            .size(18.0))
                        .clicked() {
                        self.main_menu = false;
                        self.project = Project::default();
                        self.modified = true;
                    }

                    if ui
                        .button(egui::RichText::new(format!(
                                "{} Open an existing project.",
                                egui_phosphor::FOLDER_OPEN
                            )).size(18.0))
                        .clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let file_path = path.display().to_string();
                            self.project_path = Some(path.parent().unwrap().display().to_string());
                            let file = File::open(file_path.clone()).unwrap();
                            self.project = serde_json::from_reader(file).unwrap();
                            self.main_menu = false;
                            self.status = "Opened project successfully.".to_string();
                            self.modified = false;
                        }
                    }
                });
            });
        } else {
            // Project is loaded
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.add(egui::Button::new("Save project")).clicked() {
                            // If no path is set...
                            if self.project_path.is_none() {
                                // Ask the user for a path
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    // Set the project_path to this path
                                    self.project_path = Some(path.display().to_string());
                                    // THEN save to it
                                    self.save_project();
                                } else {
                                    // Otherwise, if the user cancels selecting a path, cancel saving
                                }
                            } else {
                                // Just save to the already selected path
                                self.save_project();
                            }
                            ui.close_menu();
                        }
                        if ui.add(egui::Button::new("Exit")).clicked() {
                            if self.project_path.is_none() {
                                // Temporary
                                self.status = SAVE_FIRST.to_string();
                            } else {
                                frame.close();
                            }
                        }
                        ui.menu_button("Build and run", |ui| {
                            if ui.add(egui::Button::new("JS")).clicked() {
                                if self.project_path.is_none() {
                                    self.status = SAVE_FIRST.to_string();
                                } else {
                                    self.build_js();
                                }
                            }
                            if ui.add(egui::Button::new("Rust")).clicked() {
                                if self.project_path.is_none() {
                                    self.status = SAVE_FIRST.to_string();
                                } else {
                                    self.build_rs();
                                }
                            }
                        });
                        
                    });
                });
            });
            egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.label(self.status.clone());
                });
            });
            
            DockArea::new(&mut self.tree)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show(
                    ctx,
                    &mut TabViewer {
                        project: &mut self.project,
                        modified: &mut self.modified,
                        status: &mut self.status,
                    },
                );
        }
        
        if self.modified && self.status != SAVE_FIRST {
            self.status = MODIFIED.to_string();
        }
    }
}

impl UntitledStudioApp {
    fn save_project(&mut self) {
        match File::create(self.project_path.clone().unwrap() + "\\project.rp") {
            Ok(mut file) => {
                match std::fs::create_dir_all(std::path::Path::new(&self.project_path.clone().unwrap()).join("resources")) {
                    Ok(_) => (),
                    Err(_) => {
                        self.status = ERR_CREATING_DIR.to_string();
                        return
                    }
                }
                match file.write_all(&serde_json::to_vec(&self.project.clone()).unwrap()) {
                    Ok(_) =>  {
                        self.status = SAVE_COMPLETE.to_string();
                        self.modified = false;
                    },
                    Err(_) => {
                        self.status = ERR_WRITING.to_string();
                    }
                }
            },
            Err(_) => {
                self.status = ERR_CREATING.to_string();
            }
        }
    }
    fn build_js(&mut self) {
        let output_dir = std::path::Path::new( &self.project_path.clone().unwrap() ).join("js_output");
        match std::fs::create_dir_all(output_dir.clone()) {
            Ok(_) => (),
            Err(_) => {
                self.status = ERR_CREATING_DIR.to_string();
                return
            }
        }
        let js = File::create(output_dir.join("game.js"));
        match js {
            Ok(_) => (),
            Err(_) => {
                self.status = ERR_CREATING_JS.to_string();
                return;
            }
        }
        let html = File::create(output_dir.join("index.html"));
        match html {
            Ok(_) => (),
            Err(_) => {
                self.status = ERR_CREATING_JS.to_string();
                return;
            }
        }
        let code = crate::exporters::js::js_project(self.project.clone());
        match js.unwrap().write_all(code.1.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                self.status = ERR_WRITING_JS.to_string();
                return;
            }
        }
        match html.unwrap().write_all(code.0.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                self.status = ERR_WRITING_JS.to_string();
                return;
            }
        }
        self.status = JS_SAVED.to_string();
    }
    fn build_rs(&mut self) {
        let output_dir = std::path::Path::new( &self.project_path.clone().unwrap() ).join("rs_output");
        match std::fs::create_dir_all(output_dir.clone()) {
            Ok(_) => (),
            Err(_) => {
                self.status = ERR_CREATING_DIR.to_string();
                return
            }
        }
        let rs_project = crate::exporters::rs::rs_project(self.project.clone());
        for output_tuple in rs_project {
            let output_path = output_dir.join(output_tuple.clone().0);
            match output_tuple.clone().1 {
                Some(contents) => {
                    // File
                    let output_file = File::create(output_path);
                    match output_file {
                        Ok(_) => (),
                        Err(_) => {
                            self.status = ERR_CREATING_RS.to_string();
                            return;
                        }
                    }
                    match output_file.unwrap().write_all(contents.as_bytes()) {
                        Ok(_) => (),
                        Err(_) => {
                            self.status = ERR_WRITING_RS.to_string();
                        }
                    }
                },
                None => {
                    // Folder
                    let output_dir = output_dir.join(output_tuple.clone().0);
                    match std::fs::create_dir_all(output_dir.clone()) {
                        Ok(_) => (),
                        Err(err) => {
                            self.status = ERR_CREATING_DIR.to_string();
                            dbg!(err);
                            return
                        }
                    }
                }
            }
            self.status = RS_SAVED.to_string();
        }
    }
}
