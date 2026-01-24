use gpui::{
    prelude::*, px, size, App, Application, Bounds, WindowBounds, WindowOptions
};

mod main_window;

fn main() {
	Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| main_window::MainWindow::new())
            },
        )
        .unwrap();
    });
}
