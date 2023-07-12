use crate::project::Project;
use egui::{Label, Sense, Stroke, Color32, Pos2};

use crate::project::map::Map;
use crate::project::FocusedObjectType;

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
        });
    }
    #[allow(unused_variables, unused_mut)]
    fn map_view(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let size = ui.available_size();
                let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());
                let to_screen = emath::RectTransform::from_to(
                        egui::Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                        response.rect,
                );
                let from_screen = to_screen.inverse();
                painter.line_segment([to_screen * Pos2::new(0.0,0.0), to_screen * Pos2::new(size.x, size.y)], Stroke::new(2.0, Color32::RED));
                
                painter
                // TODO: add map drawing
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