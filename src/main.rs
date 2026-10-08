use iced::Theme;

mod window;

fn main() {
    let _ = iced::application(
        window::Window::new,
        window::Window::update,
        window::Window::view,
    )
    .theme(Theme::CatppuccinMocha)
    .subscription(window::Window::subscriptions)
    .run();
}
