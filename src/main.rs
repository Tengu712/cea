pub mod appapis;
pub mod winapis;

fn main() {
    match appapis::start_app() {
        Ok(_) => (),
        Err(e) => winapis::winapi::show_messagebox(e.get_message(), e.get_title()),
    }
}
