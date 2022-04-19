pub mod direct3d;
pub mod image;
pub mod math;
pub mod winapi;

pub struct MyErr {
    pub message: String,
    pub kind: String,
    pub place: String,
}
impl MyErr {
    pub fn new(&self) -> Self {
        Self {
            kind: String::default(),
            message: String::default(),
            place: String::default(),
        }
    }
    pub fn get_message(&self) -> String {
        self.message.clone() + " in " + self.place.as_str()
    }
    pub fn get_title(&self) -> String {
        self.kind.clone()
    }
}

pub enum EKnd {
    Common,
    Creation,
    Get,
    Io,
    Runtime,
}
pub fn errknd_string(errknd: EKnd) -> String {
    match errknd {
        EKnd::Common => String::from("Error"),
        EKnd::Creation => String::from("Creation Error"),
        EKnd::Get => String::from("Get Error"),
        EKnd::Io => String::from("IO Error"),
        EKnd::Runtime => String::from("Runtime Error"),
    }
}
