use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

#[path ="./fonts_cus.rs"]
mod fonts_cus;

#[path ="./pdf_cus.rs"]
mod pdf_cus;

pub struct MainWindow {
    pub pdf_file_path: String,
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
    pub fn new() -> Self {
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

impl Render for MainWindow {
     fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.pdf_file_path))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(gpui::red()))
                    .child(div().size_8().bg(gpui::green()))
                    .child(div().size_8().bg(gpui::blue()))
                    .child(div().size_8().bg(gpui::yellow()))
                    .child(div().size_8().bg(gpui::black()))
                    .child(div().size_8().bg(gpui::white())),
            )
    }
}