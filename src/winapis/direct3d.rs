/// [essential]
/// use winapis::math.
/// This provides data layout of constant buffer and way to update constant buffer data.
mod cbuffer;
/// use winapis::{directwrite, winapi}.
/// This enables to create DirectWrite application with Direct3D application.
mod dwd3d;
/// use winapis::{image, direct3d::cbuffer}.
/// This creates image for Direct3D from converter of WIC.
mod image;
/// This enables to draw a polygon on screen.
mod model;
/// [essential] 
/// use winapis::direct3d::cbuffer.
/// This provides easy way to call apis about shader.
mod shader;

use super::*;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::*,
            Direct3D11::*,
            Dxgi::{Common::*, *},
        },
    },
};

pub fn raise_err(errknd: EKnd, message: &str) -> WErr {
    WErr::from(errknd, String::from(message), String::from("Direct3D App"))
}
pub fn raise_err_arg(errknd: EKnd, arg: &String, message: &str) -> WErr {
    WErr::from(
        errknd,
        arg.clone() + " : " + message,
        String::from("Direct3D App"),
    )
}

pub struct D3DApplication {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    swapchain: IDXGISwapChain,
    rtv_bbuf: Option<ID3D11RenderTargetView>,
    cbuffer: Option<ID3D11Buffer>,
}
impl D3DApplication {
    pub fn new(
        winapp: &super::winapi::WindowsApplication,
        width: u32,
        height: u32,
    ) -> Result<Self, WErr> {
        // Create factory
        let factory = unsafe {
            CreateDXGIFactory::<IDXGIFactory>()
                .map_err(|_| raise_err(EKnd::Creation, "DXGIFactory"))?
        };
        // Create device
        let (device, context) = unsafe {
            let mut ppdevice = None;
            let mut ppimmediatecontext = None;
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HINSTANCE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                &D3D_FEATURE_LEVEL_11_1,
                1,
                D3D11_SDK_VERSION,
                &mut ppdevice,
                std::ptr::null_mut(),
                &mut ppimmediatecontext,
            )
            .map_err(|_| raise_err(EKnd::Common, "D3D11CreateDevice failed"))?;
            (
                ppdevice.ok_or(raise_err(EKnd::Creation, "D3D11Device"))?,
                ppimmediatecontext.ok_or(raise_err(EKnd::Creation, "D3D11DeviceContext"))?,
            )
        };
        // Create swapchain
        let swapchain = unsafe {
            let buffer_desc = DXGI_MODE_DESC {
                Width: width,
                Height: height,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 60,
                    Denominator: 1,
                },
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
            };
            let pdesc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: buffer_desc,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 1,
                OutputWindow: *winapp.get_window_handle(),
                Windowed: BOOL(1),
                SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                Flags: 2,
            };
            factory
                .CreateSwapChain(&device, &pdesc)
                .map_err(|_| raise_err(EKnd::Creation, "SwapChain"))?
        };
        // Create back buffer rtv
        let rtv_bbuf = unsafe {
            let bbuf = swapchain
                .GetBuffer::<ID3D11Texture2D>(0)
                .map_err(|_| raise_err(EKnd::Get, "RTV of backbuffer"))?;
            device
                .CreateRenderTargetView(bbuf, std::ptr::null())
                .map_err(|_| raise_err(EKnd::Creation, "RTV of backbuffer"))?
        };
        // Create shaders
        let shader_coms = shader::ShaderComs::new(&device, winapp.get_cur_dir())?;
        // Set render configure
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: width as f32,
            Height: height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        unsafe { context.RSSetViewports(1, &viewport) };
        let blend_desc = {
            let mut render_targets = [D3D11_RENDER_TARGET_BLEND_DESC::default(); 8];
            render_targets[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: BOOL(1),
                SrcBlend: D3D11_BLEND_SRC_ALPHA,
                DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
                BlendOp: D3D11_BLEND_OP_ADD,
                SrcBlendAlpha: D3D11_BLEND_ONE,
                DestBlendAlpha: D3D11_BLEND_ONE,
                BlendOpAlpha: D3D11_BLEND_OP_ADD,
                RenderTargetWriteMask: 0b1111,
            };
            D3D11_BLEND_DESC {
                AlphaToCoverageEnable: BOOL(0),
                IndependentBlendEnable: BOOL(0),
                RenderTarget: render_targets,
            }
        };
        let blend_state = unsafe {
            device
                .CreateBlendState(&blend_desc)
                .map_err(|_| raise_err(EKnd::Creation, "BlendState"))?
        };
        unsafe { context.OMSetBlendState(blend_state, [1.0, 1.0, 1.0, 1.0].as_ptr(), 0xffffffff) };
        unsafe { context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST) };
        unsafe { context.IASetInputLayout(shader_coms.ilayout) };
        unsafe { context.VSSetShader(shader_coms.vshader, std::ptr::null(), 0) };
        unsafe { context.PSSetShader(shader_coms.pshader, std::ptr::null(), 0) };
        // Finish
        Ok(Self {
            device,
            context,
            swapchain,
            rtv_bbuf: Some(rtv_bbuf),
            cbuffer: Some(shader_coms.cbuffer),
        })
    }
    /// Set render target view.
    pub fn set_rtv(&self) {
        unsafe { self.context.OMSetRenderTargets(1, &self.rtv_bbuf, None) };
    }
    /// Clear render target view.
    pub fn clear_rtv(&self) {
        unsafe {
            self.context
                .ClearRenderTargetView(&self.rtv_bbuf, [0.0, 0.0, 0.0, 1.0].as_ptr())
        };
    }
    /// Swap and wait vsync.
    pub fn swap(&self) -> Result<(), WErr> {
        unsafe {
            self.swapchain
                .Present(1, 0)
                .map_err(|_| raise_err(EKnd::Runtime, "Backbuffer swap failed"))
        }
    }
}
