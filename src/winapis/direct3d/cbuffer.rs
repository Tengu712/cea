use super::{super::math::Matrix4x4, *};

/// Constant buffer data.
pub struct CData {
    pub mat_scl: Matrix4x4,
    pub mat_rtx: Matrix4x4,
    pub mat_rty: Matrix4x4,
    pub mat_rtz: Matrix4x4,
    pub mat_trs: Matrix4x4,
    pub mat_view: Matrix4x4,
    pub mat_proj: Matrix4x4,
    pub vec_col: [f32; 4],
    pub vec_prm: [f32; 4],
}

impl D3DApplication {
    /// Set constant data to constant buffer.
    pub fn set_cdata(&self, cdata: &CData) -> Result<(), WErr> {
        unsafe {
            self.context.UpdateSubresource(
                self.cbuffer
                    .as_ref()
                    .ok_or(raise_err(EKnd::Runtime, "Cbuffer is None"))?,
                0,
                std::ptr::null(),
                std::mem::transmute(cdata),
                0,
                0,
            );
            self.context.VSSetConstantBuffers(0, 1, &self.cbuffer);
        };
        Ok(())
    }
}
