use super::*;
use windows::Win32::{
    Graphics::Imaging::*,
    System::{
        Com::{CoCreateInstance, CLSCTX_SERVER},
        SystemServices::GENERIC_READ,
    },
};

fn raise_err(errknd: EKnd, arg: &String, message: &str) -> WErr {
    WErr::from(
        errknd,
        arg.clone() + " : " + message,
        String::from("Windows Imaging Component"),
    )
}

/// Struct to reference image data.
pub(super) struct ImageConverter {
    pub(super) converter: IWICFormatConverter,
    pub(super) width: u32,
    pub(super) height: u32,
}
impl ImageConverter {
    pub(super) fn from_file(path: String) -> Result<Self, WErr> {
        let factory: IWICImagingFactory = unsafe {
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_SERVER)
                .map_err(|_| raise_err(EKnd::Creation, &path, "WICfactory"))?
        };
        let decoder = unsafe {
            factory
                .CreateDecoderFromFilename(
                    path.clone(),
                    std::ptr::null(),
                    GENERIC_READ,
                    WICDecodeMetadataCacheOnLoad,
                )
                .map_err(|_| raise_err(EKnd::Creation, &path, "Decoder"))?
        };
        let frame = unsafe {
            decoder
                .GetFrame(0)
                .map_err(|_| raise_err(EKnd::Get, &path, "Frame"))?
        };
        let converter = unsafe {
            factory
                .CreateFormatConverter()
                .map_err(|_| raise_err(EKnd::Creation, &path, "Format converter"))?
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
                .map_err(|_| raise_err(EKnd::Common, &path, "Initialize converter"))?
        };
        let mut width = 0;
        let mut height = 0;
        unsafe {
            converter
                .GetSize(&mut width, &mut height)
                .map_err(|_| raise_err(EKnd::Get, &path, "Size"))?
        };
        Ok(Self {
            converter,
            width,
            height,
        })
    }
}
