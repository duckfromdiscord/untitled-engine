use std::ops::Add;

use egui::{Label, Sense, Color32, Pos2, Vec2, Rounding, Stroke};

use untitled_ir::project::map::Map;
use untitled_ir::project::FocusedObjectType;
use untitled_ir::project::Project;

pub struct TabViewer<'a> {
    pub project: &'a mut Project,
    pub modified: &'a mut bool,
    pub status: &'a mut String,
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
                    *self.modified = true;
                }
            });
            ui.collapsing("Resources", |ui| {
                if ui.button(egui::RichText::new(egui_phosphor::FOLDER_PLUS.to_string()).size(16.0)).clicked() {
                    
                }
            });
        });
    }
    #[allow(unused_variables, unused_mut)]
    fn map_view(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {

                let current_map = &self.project.maps[self.project.selected_map_index];
                let map_size = Vec2::new(current_map.size.0.into(), current_map.size.1.into());
                let map_color = Color32::from_rgb(current_map.color.r, current_map.color.g, current_map.color.b);

                let size = ui.available_size();
                let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());
                let to_screen = emath::RectTransform::from_to(
                    egui::Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                        response.rect,
                );
                let from_screen = to_screen.inverse();
                
                let window_center = to_screen * Pos2::new( 20.0,20.0);

                // Editor background
                painter.rect_filled([to_screen * Pos2::new(0.0,0.0), to_screen * Pos2::new(size.x, size.y)].into(), Rounding::none(), Color32::from_gray(20));
                
                // Map background
                painter.rect_filled([window_center, window_center.add(map_size)].into(), Rounding::none(), map_color);
                
                
                painter
        });
    }
    fn object_view(&mut self, ui: &mut egui::Ui) {
        match self.project.focused_object_type {
            FocusedObjectType::Project => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Project Name");
                        ui.text_edit_singleline(&mut self.project.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Window Size");
                        let mut width: String = self.project.window_size.0.to_string();
                        let mut height: String = self.project.window_size.1.to_string();
                        ui.add(egui::TextEdit::singleline(&mut width).desired_width(90.0));
                        ui.add(egui::TextEdit::singleline(&mut height).desired_width(90.0));
                        self.project.window_size.0 = width.parse::<usize>().unwrap_or(1080);
                        self.project.window_size.1 = height.parse::<usize>().unwrap_or(720);
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