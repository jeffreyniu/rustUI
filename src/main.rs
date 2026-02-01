mod main_window;

fn main() -> iced::Result {
     iced::application(main_window::Styling::default, main_window::Styling::update, main_window::Styling::view)
        .subscription(main_window::Styling::subscription)
        .theme(main_window::Styling::theme)
        .run()
}
