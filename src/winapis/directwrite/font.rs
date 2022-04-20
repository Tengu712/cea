use super::*;

impl DWriteApp {
    /// Register custom font.
    pub fn register_custom_font(&self, path: String) -> Result<(), WErr> {
        let font_file = unsafe {
            self.dwfactory
                .CreateFontFileReference(path.clone(), std::ptr::null())
                .map_err(|_| raise_err_arg(EKnd::Creation, &path, "FontFile"))?
        };
        let font_set_builder = unsafe {
            self.dwfactory
                .CreateFontSetBuilder2()
                .map_err(|_| raise_err_arg(EKnd::Creation, &path, "FontSetBuilder"))?
        };
        unsafe {
            font_set_builder
                .AddFontFile(font_file)
                .map_err(|_| raise_err_arg(EKnd::Common, &path, "Add font file failed"))?
        };
        let font_set = unsafe {
            font_set_builder
                .CreateFontSet()
                .map_err(|_| raise_err_arg(EKnd::Creation, &path, "FontSet"))?
        };
        let font_collection = unsafe {
            self.dwfactory
                .CreateFontCollectionFromFontSet(font_set)
                .map_err(|_| raise_err_arg(EKnd::Creation, &path, "FontCollection"))?
        };
        /*
        unsafe {
            self.dwfactory
                .RegisterFontCollectionLoader(font_collection)
                .map_err(|_| {
                    raise_err_arg(EKnd::Common, &path, "Registration font collection failed")
                })?
        };*/
        Ok(())
    }
}
