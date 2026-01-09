use eframe::egui;

pub struct MainWindow {
    name: String,
    age: u32,
    gender: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            gender: "Male".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut visuals = egui::Visuals::light();
        //visuals.widgets.inactive.bg_fill = egui::Color32::WHITE;
        //visuals.widgets.hovered.bg_fill  = egui::Color32::WHITE;
        //visuals.widgets.active.bg_fill   = egui::Color32::WHITE;
        
        // use a consistent gray border for all widget states
        let red = egui::Color32::from_rgb(160,0,0);
        //visuals.widgets.inactive.bg_stroke = egui::Stroke::new(2.0, red);
        visuals.widgets.hovered.bg_stroke  = egui::Stroke::new(2.0, red);
        visuals.widgets.active.bg_stroke   = egui::Stroke::new(2.0, red);

        // style selection (used by Slider filled part and selection visuals)
        visuals.selection.bg_fill = egui::Color32::from_rgb(0,160,0);
        visuals.selection.stroke = egui::Stroke::new(2.0, red);

        visuals.window_fill = egui::Color32::WHITE; // makes popup windows white too
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
        //.frame(egui::Frame::default().fill(egui::Color32::WHITE))     
        .show(ctx, |ui| {
            ui.heading("self-introduction");
            ui.horizontal(|ui| {
                ui.label("name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui|{   
                ui.label("gender:");                    
                egui::ComboBox::new("cboxGender", "")
                .selected_text(self.gender.clone())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.gender, "Male".to_owned(), "Male");
                    ui.selectable_value(&mut self.gender, "Female".to_owned(), "Female");
                });               
            });
            ui.horizontal(|ui|{
                ui.label("age: ");
                let slider = egui::Slider::new(&mut self.age, 0..=120).show_value(false);
                ui.add(slider);

                if ui.button("add one").clicked() {
                    self.age += 1;
                }
            });
        ui.label(format!("I'm {}, {} years old, {}", self.name, self.age, self.gender));
        });
    }
}
