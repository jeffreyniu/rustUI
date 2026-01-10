#[path ="./fonts_cus.rs"]
mod fonts_cus;

#[path ="./pdf_cus.rs"]
mod pdf_cus;

use eframe::egui;

pub struct MainWindow {
    pdf_file_path: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            pdf_file_path: "/home/niutb/文档/jeffreyniu/Hillbilly Elegy by J. D. Vance.pdf".to_string()
        }
    }
}

impl MainWindow {
    // 构造方法（接收创建上下文）
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts_cus::load_fonts(&cc.egui_ctx); // 初始化字体系统
        Self::default() // 返回默认实例
    }

    fn read_pdf(&self, _path: &str) {
        let result = pdf_cus::load_pdf_of_hands_language(_path);

        match result {
            Ok(info) => {
                for (index, item) in info.iter().enumerate() {
                    println!("PDF Info: {} - {}",index, item);
                }
            }
            Err(e) => {
                println!("Error reading PDF or saving images: {:?}", e);
            }
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut visuals = egui::Visuals::light();
        
        // use a consistent gray border for all widget states
        let red = egui::Color32::from_rgb(160,0,0);
        visuals.widgets.hovered.bg_stroke  = egui::Stroke::new(1.0, red);
        visuals.widgets.active.bg_stroke   = egui::Stroke::new(1.0, red);

        // style selection (used by Slider filled part and selection visuals)
        visuals.selection.bg_fill = egui::Color32::from_rgb(0,100,0);
        visuals.selection.stroke = egui::Stroke::new(1.0, red);

        visuals.window_fill = egui::Color32::WHITE; // makes popup windows white too
        ctx.set_visuals(visuals);
        
        egui::CentralPanel::default()  
        .show(ctx, |ui| {
            ui.heading("Tools");
            
            ui.horizontal(|ui| {
                ui.label("PDF file: ");

                ui.add(egui::TextEdit::singleline(&mut self.pdf_file_path).desired_width(450.0));

                if ui.button("read").clicked() {
                    self.read_pdf(&self.pdf_file_path);
                }
            });
        });
    }
}