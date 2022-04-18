pub mod direct2d;
pub mod direct3d;
pub mod math;
pub mod winapi;

pub enum ErrKnd {
    Common,
    Creation,
    Get,
    Io,
    Runtime,
}

pub enum MyErr {
    WinApp(ErrKnd, String),
    D3DApp(ErrKnd, String),
    App(ErrKnd, String),
}

fn errknd_str(k: ErrKnd) -> String {
    match k {
        ErrKnd::Common => String::from("Error"),
        ErrKnd::Creation => String::from("Creation Error"),
        ErrKnd::Get => String::from("Get Error"),
        ErrKnd::Io => String::from("IO Error"),
        ErrKnd::Runtime => String::from("Runtime Error"),
    }
}

pub fn myerr_msg_ttl(e: MyErr) -> (String, String) {
    match e {
        MyErr::WinApp(k, m) => (m + " in WinApp", errknd_str(k)),
        MyErr::D3DApp(k, m) => (m + " in D3DApp", errknd_str(k)),
        MyErr::App(k, m) => (m, errknd_str(k)),
    }
}
