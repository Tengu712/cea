use super::*;
use std::mem::size_of;

/// Use it when making model buffer.
pub struct Vertex {
    pub pos: [f32; 3],
    pub col: [f32; 4],
    pub tex: [f32; 2],
}

/// A struct for vertex and index buffers.
pub struct ModelBuffer {
    num_idx: u32,
    vbuf: Option<ID3D11Buffer>,
    ibuf: Option<ID3D11Buffer>,
}

impl D3DApplication {
    /// Create model buffer.
    pub fn create_modelbuffer(
        &self,
        num_vtx: u32,
        data_vtx: &[Vertex],
        num_idx: u32,
        data_idx: &[u32],
    ) -> Result<ModelBuffer, WErr> {
        let vbuf = unsafe {
            let vbuf_desc = D3D11_BUFFER_DESC {
                ByteWidth: size_of::<Vertex>() as u32 * num_vtx,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: 1,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let vbuf_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: std::mem::transmute(&data_vtx),
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            self.device
                .CreateBuffer(&vbuf_desc, &vbuf_data)
                .map_err(|_| raise_err(EKnd::Creation, "Model vbuffer"))?
        };
        let ibuf = unsafe {
            let ibuf_desc = D3D11_BUFFER_DESC {
                ByteWidth: size_of::<u32>() as u32 * num_idx,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: 2,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            let ibuf_data = D3D11_SUBRESOURCE_DATA {
                pSysMem: std::mem::transmute(&data_idx),
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            self.device
                .CreateBuffer(&ibuf_desc, &ibuf_data)
                .map_err(|_| raise_err(EKnd::Creation, "Model ibuffer"))?
        };
        Ok(ModelBuffer {
            num_idx,
            vbuf: Some(vbuf),
            ibuf: Some(ibuf),
        })
    }
    /// Draw model on current buffer.
    pub fn draw_model(&self, mbuf: &ModelBuffer) -> Result<(), WErr> {
        unsafe {
            self.context
                .IASetVertexBuffers(0, 1, &mbuf.vbuf, &(size_of::<Vertex>() as u32), &0);
            self.context
                .IASetIndexBuffer(&mbuf.ibuf, DXGI_FORMAT_R32_UINT, 0);
            self.context.DrawIndexed(mbuf.num_idx, 0, 0);
        };
        Ok(())
    }
}
