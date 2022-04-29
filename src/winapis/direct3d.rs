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
