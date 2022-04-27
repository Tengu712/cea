use windows::{
    core::Result,
    Win32::{
        Graphics::Imaging::*,
        System::{
            Com::{CoCreateInstance, CLSCTX_SERVER},
            SystemServices::GENERIC_READ,
        },
    },
};

/// Struct to reference image data.
pub(super) struct ImageConverter {
    pub(super) converter: IWICFormatConverter,
    pub(super) width: u32,
    pub(super) height: u32,
}
impl ImageConverter {
    pub(super) fn from_file(path: String) -> Result<Self> {
        let factory: IWICImagingFactory =
            unsafe { CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_SERVER)? };
        let decoder = unsafe {
            factory.CreateDecoderFromFilename(
                path.clone(),
                std::ptr::null(),
                GENERIC_READ,
                WICDecodeMetadataCacheOnLoad,
            )?
        };
        let frame = unsafe { decoder.GetFrame(0)? };
        let converter = unsafe { factory.CreateFormatConverter()? };
        unsafe {
            converter.Initialize(
                frame,
                &GUID_WICPixelFormat32bppRGBA,
                WICBitmapDitherTypeNone,
                None,
                1.0,
                WICBitmapPaletteTypeMedianCut,
            )?
        };
        let mut width = 0;
        let mut height = 0;
        unsafe { converter.GetSize(&mut width, &mut height)? };
        Ok(Self {
            converter,
            width,
            height,
        })
    }
}
