mod main_window;

fn main() -> iced::Result {
     iced::application(main_window::MainWindow::default, main_window::MainWindow::update, main_window::MainWindow::view)
        .font(include_bytes!("../fonts/wqy-zenhei.ttc").as_slice())
        .subscription(main_window::MainWindow::subscription)
        .theme(main_window::MainWindow::theme)
        .run()
}
