use crate::project::Project;

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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                            self.project_path = Some(path.display().to_string());
                            match File::open(self.project_path.clone().unwrap()) {
                                Ok(file) => {
                                    self.project = serde_json::from_reader(file).unwrap();
                                    self.main_menu = false;
                                    self.status = "Opened project successfully.".to_string();
                                    self.modified = false;
                                },
                                Err(_) => {

                                }
                            }
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
                                if let Some(path) = rfd::FileDialog::new().save_file() {
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
                            todo!();
                        }
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
        
        if self.modified {
            self.status = "Modified. Please save.".to_string();
        }
    }
}

impl UntitledStudioApp {
    fn save_project(&mut self) {
        match File::create(self.project_path.clone().unwrap()) {
            Ok(mut file) => {
                match file.write_all(&serde_json::to_vec(&self.project.clone()).unwrap()) {
                    Ok(_) =>  {
                        self.status = "Save complete.".to_string();
                        self.modified = false;
                    },
                    Err(_) => {
                        self.status = "Error writing to file. Your data is NOT saved.".to_string();
                    }
                }
            },
            Err(_) => {
                self.status = "Error creating file to save to. Your data is NOT saved.".to_string();
            }
        }
    }
}
