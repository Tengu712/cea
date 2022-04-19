use super::{super::image::ImageConverter, cbuffer::CData, *};

/// Image available to Direct3D.
pub struct D3DImage {
    srv_img: Option<ID3D11ShaderResourceView>,
    pub width: u32,
    pub height: u32,
}

impl D3DApplication {
    /// Create Image for Direct3D.
    pub fn create_image_from_file(&self, path: &str) -> Result<D3DImage, MyErr> {
        let img_cnvtr = ImageConverter::from_file(path)?;
        let texture = unsafe {
            let tex_desc = D3D11_TEXTURE2D_DESC {
                Width: img_cnvtr.width,
                Height: img_cnvtr.height,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_SHADER_RESOURCE,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
                MiscFlags: D3D11_RESOURCE_MISC_FLAG(0),
            };
            self.device
                .CreateTexture2D(&tex_desc, std::ptr::null())
                .map_err(|_| MyErr::d3d_arg(EKnd::Creation, path, "Image texture"))?
        };
        unsafe {
            let mappd_sres = self
                .context
                .Map(&texture, 0, D3D11_MAP_WRITE_DISCARD, 0)
                .map_err(|_| MyErr::d3d_arg(EKnd::Common, path, "Map texture"))?;
            img_cnvtr
                .converter
                .CopyPixels(
                    std::ptr::null(),
                    img_cnvtr.width * 4,
                    img_cnvtr.width * img_cnvtr.height * 4,
                    mappd_sres.pData as *mut u8,
                )
                .map_err(|_| MyErr::d3d_arg(EKnd::Common, path, "Copy pixels"))?;
            self.context.Unmap(&texture, 0);
        };
        let srv_img = unsafe {
            let srv_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
                Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2D: D3D11_TEX2D_SRV {
                        MostDetailedMip: 0,
                        MipLevels: 1,
                    },
                },
            };
            self.device
                .CreateShaderResourceView(&texture, &srv_desc)
                .map_err(|_| MyErr::d3d_arg(EKnd::Creation, path, "ShaderResourceView"))?
        };
        Ok(D3DImage {
            srv_img: Some(srv_img),
            width: img_cnvtr.width,
            height: img_cnvtr.height,
        })
    }
    /// Set D3DImage.
    pub fn set_d3dimage(&self, d3dimage: Option<&D3DImage>, cdata: CData) -> CData {
        let mut cdata_mut = cdata;
        if let Some(n) = d3dimage {
            unsafe { self.context.PSSetShaderResources(0, 1, &n.srv_img) };
            cdata_mut.vec_prm[0] = 1.0;
        } else {
            cdata_mut.vec_prm[0] = 0.0;
        }
        cdata_mut
    }
}
