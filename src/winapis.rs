/// use many modules.
/// This enables to react Computer Graphics by Direct3D11.
pub mod direct3d;
/// This enables to draw text on screen.
pub mod directwrite;
/// This's to create image converter of WIC. User can craete image with from it.
mod image;
/// This provides Matrix4x4 that user can create as affine matrix.
pub mod math;
/// This's to crate window or use common WindowsAPI.
pub mod winapi;

pub struct WErr {
    kind: String,
    message: String,
    place: String,
}
impl WErr {
    fn from(errknd: EKnd, message: String, place: String) -> Self {
        Self {
            kind: errknd_string(errknd),
            message,
            place,
        }
    }
    pub fn get_message(&self) -> String {
        self.message.clone() + " in " + self.place.as_str()
    }
    pub fn get_title(&self) -> String {
        self.kind.clone()
    }
}

enum EKnd {
    Common,
    Creation,
    Get,
    Io,
    Runtime,
}
fn errknd_string(errknd: EKnd) -> String {
    match errknd {
        EKnd::Common => String::from("Error"),
        EKnd::Creation => String::from("Creation Error"),
        EKnd::Get => String::from("Get Error"),
        EKnd::Io => String::from("IO Error"),
        EKnd::Runtime => String::from("Runtime Error"),
    }
}
