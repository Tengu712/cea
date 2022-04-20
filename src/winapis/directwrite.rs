/// This enables to use custom font.
pub mod font;
/// This enables to draw text on screen.
pub mod text;

use super::*;
use windows::Win32::Graphics::{Direct2D::*, DirectWrite::*};

pub fn raise_err(errknd: EKnd, message: &str) -> WErr {
    WErr::from(
        errknd,
        String::from(message),
        String::from("DirectWrite App"),
    )
}
pub fn raise_err_arg(errknd: EKnd, arg: &String, message: &str) -> WErr {
    WErr::from(
        errknd,
        arg.clone() + " : " + message,
        String::from("DirectWrite App"),
    )
}

pub struct DWriteApp {
    d2context: ID2D1DeviceContext,
    dwfactory: IDWriteFactory5,
    bitmap: ID2D1Bitmap1,
}
impl DWriteApp {
    pub fn from(
        d2context: ID2D1DeviceContext,
        dwfactory: IDWriteFactory5,
        bitmap: ID2D1Bitmap1,
    ) -> Self {
        Self {
            d2context,
            dwfactory,
            bitmap,
        }
    }
}
