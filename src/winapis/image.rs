use super::*;
use windows::Win32::{
    Graphics::Imaging::*,
    System::{
        Com::{CoCreateInstance, CLSCTX_SERVER},
        SystemServices::GENERIC_READ,
    },
};

impl MyErr {
    fn wic(errknd: EKnd, arg: &str, message: &str) -> Self {
        Self {
            message: String::from(arg) + " : " + message,
            kind: errknd_string(errknd),
            place: String::from("WIC"),
        }
    }
}

/// Struct to reference image data.
pub struct ImageConverter {
    pub converter: IWICFormatConverter,
    pub width: u32,
    pub height: u32,
}
impl ImageConverter {
    pub fn from_file(path: &str) -> Result<Self, MyErr> {
        let factory: IWICImagingFactory = unsafe {
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_SERVER)
                .map_err(|_| MyErr::wic(EKnd::Creation, path, "WICfactory"))?
        };
        let decoder = unsafe {
            factory
                .CreateDecoderFromFilename(
                    path,
                    std::ptr::null(),
                    GENERIC_READ,
                    WICDecodeMetadataCacheOnLoad,
                )
                .map_err(|_| MyErr::wic(EKnd::Creation, path, "Decoder"))?
        };
        let frame = unsafe {
            decoder
                .GetFrame(0)
                .map_err(|_| MyErr::wic(EKnd::Get, path, "Frame"))?
        };
        let converter = unsafe {
            factory
                .CreateFormatConverter()
                .map_err(|_| MyErr::wic(EKnd::Creation, path, "Format converter"))?
        };
        unsafe {
            converter
                .Initialize(
                    frame,
                    &GUID_WICPixelFormat32bppRGBA,
                    WICBitmapDitherTypeNone,
                    None,
                    1.0,
                    WICBitmapPaletteTypeMedianCut,
                )
                .map_err(|_| MyErr::wic(EKnd::Common, path, "Initialize converter"))?
        };
        let mut width = 0;
        let mut height = 0;
        unsafe {
            converter
                .GetSize(&mut width, &mut height)
                .map_err(|_| MyErr::wic(EKnd::Get, path, "Size"))?
        };
        Ok(Self {
            converter,
            width,
            height,
        })
    }
}
