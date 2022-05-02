/// [essential]
/// use winapis::math.
/// This provides data layout of constant buffer and way to update constant buffer data.
pub mod cbuffer;
/// use winapis::{directwrite, winapi}.
/// This enables to create DirectWrite application with Direct3D application.
mod dwd3d;
/// [essential]
/// This provides a way to create D3DApplication and clean screen.
mod general;
/// use winapis::{image, direct3d::cbuffer}.
/// This creates image for Direct3D from converter of WIC.
pub mod image;
/// This enables to draw a polygon on screen.
pub mod model;
/// [essential]
/// use winapis::direct3d::cbuffer.
/// This provides easy way to call apis about shader.
mod shader;

pub struct D3DApplication {
    device: windows::Win32::Graphics::Direct3D11::ID3D11Device,
    context: windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext,
    swapchain: windows::Win32::Graphics::Dxgi::IDXGISwapChain,
    rtv_bbuf: Option<windows::Win32::Graphics::Direct3D11::ID3D11RenderTargetView>,
    dsview: windows::Win32::Graphics::Direct3D11::ID3D11DepthStencilView,
    cbuffer: Option<windows::Win32::Graphics::Direct3D11::ID3D11Buffer>,
}

/// Create idea sprite.
pub fn create_idea(d3dapp: &D3DApplication) -> Result<model::ModelBuffer, windows::core::Error> {
    let data_vtx = [
        model::Vertex {
            pos: [-0.5, -0.5, 0.0],
            col: [1.0; 4],
            tex: [0.0, 1.0],
        },
        model::Vertex {
            pos: [-0.5, 0.5, 0.0],
            col: [1.0; 4],
            tex: [0.0, 0.0],
        },
        model::Vertex {
            pos: [0.5, 0.5, 0.0],
            col: [1.0; 4],
            tex: [1.0, 0.0],
        },
        model::Vertex {
            pos: [0.5, -0.5, 0.0],
            col: [1.0; 4],
            tex: [1.0, 1.0],
        },
    ];
    let data_idx = [0, 1, 2, 0, 2, 3];
    d3dapp.create_modelbuffer(4, &data_vtx, 6, &data_idx)
}
/// Create default constant buffer data.
pub fn create_default_cdata(width: f32, height: f32) -> cbuffer::CData {
    cbuffer::CData {
        mat_scl: super::math::Matrix4x4::new_identity(),
        mat_rtx: super::math::Matrix4x4::new_identity(),
        mat_rty: super::math::Matrix4x4::new_identity(),
        mat_rtz: super::math::Matrix4x4::new_identity(),
        mat_trs: super::math::Matrix4x4::new_identity(),
        mat_view: super::math::Matrix4x4::new_identity(),
        mat_proj: super::math::Matrix4x4::new_ortho(0.0, width, height, 0.0, 0.0, 1000.0),
        vec_col: [1.0; 4],
        vec_prm: [0.0; 4],
    }
}
