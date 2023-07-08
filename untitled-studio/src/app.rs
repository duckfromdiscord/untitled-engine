#[derive(Default)]
pub struct UntitledStudioApp {
}


impl UntitledStudioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for UntitledStudioApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

         egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.horizontal(|ui| {
                
                
                
            });
        });

    }



    
}
