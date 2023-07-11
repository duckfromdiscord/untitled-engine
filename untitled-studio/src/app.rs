use crate::project::Project;
use egui::{Label, Sense};


use egui_dock::{DockArea, NodeIndex, Style, Tree};

use crate::project::map::Map;
use crate::project::FocusedObjectType;

use std::{io::Write, fs::File};

struct TabViewer<'a> {
    project: &'a mut Project,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "Project Tree" => {
                self.project_tree(ui);
            },
            "Map View" => {
                self.map_view(ui);
            },
            "Object View" => {
                self.object_view(ui);
            }
            _ => {
                ui.label(tab.as_str());
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}

impl TabViewer<'_> {
    fn project_tree(&mut self, ui: &mut egui::Ui) {
        // Layout of the project

        ui.collapsing("Opened project", |ui| {
            // Project itself
            if ui.add(Label::new(self.project.name.clone()).sense(Sense::click())).clicked() {
                self.project.focused_object_type = FocusedObjectType::Project;
                self.project.focused_object_index = 0;
            }
            // Objects in the project
            ui.collapsing("Objects", |_ui| {

            });
            // Maps in the project
            ui.collapsing("Maps", |ui| {
                let mut index: usize = 0;
                for map in &mut self.project.maps {
                    ui.horizontal(|ui| {
                        if map.internals.name_selected {
                            map.internals.name_selected = !ui.add(egui::TextEdit::singleline(&mut map.name).desired_width(128.0)).clicked_elsewhere();
                        } else {
                            let response = ui.add(Label::new(map.name.clone()).sense(Sense::click()));
                            map.internals.name_selected = response.double_clicked();
                            if response.clicked() {
                                self.project.focused_object_type = FocusedObjectType::Map;
                                self.project.focused_object_index = index;
                            }
                        }

                        if self.project.selected_map_index == index {
                            ui.button(egui::RichText::new(egui_phosphor::FOLDER_OPEN.to_string()).size(16.0)).clicked();
                        } else if ui.button(egui::RichText::new(egui_phosphor::FOLDER.to_string()).size(16.0)).clicked() {
                                self.project.selected_map_index = index;
                        }
                    });
                    index += 1;
                }
                if ui.button(egui::RichText::new("New")).clicked() {
                    self.project
                        .maps
                        .push(Map::new("Map ".to_owned() + &index.to_string()));
                }
            });
        });
    }
    fn map_view(&mut self, _ui: &mut egui::Ui) {
        
    }
    fn object_view(&mut self, ui: &mut egui::Ui) {
        match self.project.focused_object_type {
            FocusedObjectType::Project => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Project Name");
                        ui.text_edit_singleline(&mut self.project.name);
                    });
                });
            },
            FocusedObjectType::Map => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Map Name");
                        ui.text_edit_singleline(&mut self.project.maps[self.project.focused_object_index].name);
                    });
                });
            },
            FocusedObjectType::Object => {

            }
        }
    }
}

pub struct UntitledStudioApp {
    // Main menu
    main_menu: bool,
    project: Project,
    project_path: Option<String>,
    // For a loaded project
    tree: Tree<String>,
    status: String,
}

impl Default for UntitledStudioApp {
    fn default() -> Self {
        let mut tree = Tree::new(vec!["Map View".to_owned()]);

        // You can modify the tree before constructing the dock
        let [_, b] = tree.split_left(NodeIndex::root(), 0.3, vec!["Project Tree".to_owned()]);
        let [_, _] = tree.split_below(b, 0.5, vec!["Object View".to_owned()]);

        UntitledStudioApp {
            main_menu: true,
            project: Project::default(),
            project_path: None,
            tree,
            status: "Ok.".to_string(),
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
                        .button(
                            egui::RichText::new(format!(
                                "{} Create a new project.",
                                egui_phosphor::FOLDER_PLUS
                            ))
                            .size(18.0),
                        )
                        .clicked()
                    {
                        self.main_menu = false;
                        self.project = Project::default();
                    }

                    if ui
                        .button(
                            egui::RichText::new(format!(
                                "{} Open an existing project.",
                                egui_phosphor::FOLDER_OPEN
                            ))
                            .size(18.0),
                        )
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.project_path = Some(path.display().to_string());
                            match File::open(self.project_path.clone().unwrap()) {
                                Ok(file) => {
                                    self.project = serde_json::from_reader(file).unwrap();
                                    self.main_menu = false;
                                    self.status = "Opened project successfully.".to_string();
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
                                    match File::create(self.project_path.clone().unwrap()) {
                                        Ok(mut file) => {
                                            match file.write_all(&serde_json::to_vec(&self.project.clone()).unwrap()) {
                                                Ok(_) => (),
                                                Err(_) => {
                                                    self.status = "Error writing to file. Your data is NOT saved.".to_string();
                                                }
                                            }
                                        },
                                        Err(_) => {
                                            self.status = "Error creating file to save to. Your data is NOT saved.".to_string();
                                        }
                                    }
                                    
                                } else {
                                    // Otherwise, if the user cancels selecting a path, cancel saving
                                }
                            } else {
                                // Just save to the already selected path
                            }
                        }
                        if ui.add(egui::Button::new("Exit")).clicked() {
                            
                        }
                    });
                });
            });

            DockArea::new(&mut self.tree)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show(
                    ctx,
                    &mut TabViewer {
                        project: &mut self.project,
                    },
                );
            
            egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.label(self.status.clone());
                });
            });
        }
    }
}
