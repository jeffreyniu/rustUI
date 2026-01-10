#[path ="./fonts_cus.rs"]
mod fonts_cus;

use eframe::egui;

pub struct MainWindow {
    pdf_file_path: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            pdf_file_path: "/home/niutb/文档/jeffreyniu/GF0020-2018《国家通用手语常用词表》全本.pdf".to_string()
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
        // Placeholder for PDF reading logic
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
                ui.text_edit_singleline(&mut self.pdf_file_path);
                if ui.button("read").clicked() {
                    self.read_pdf(&self.pdf_file_path);
                }
            });
        });
    }
}