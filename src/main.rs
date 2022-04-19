pub mod app;
pub mod winapis;

fn main() {
    match app::start_app() {
        Ok(_) => (),
        Err(e) => winapis::winapi::show_messagebox(e.get_message(), e.get_title()),
    }
}
