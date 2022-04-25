use super::cbuffer::CData;
use super::*;
use std::{fs::File, io::Read, mem::size_of};
use windows::{
    core::PCSTR,
    Win32::Graphics::{Direct3D11::*, Dxgi::Common::*},
};

/// A struct for giving coms around shaders to D3DApp.
pub(super) struct ShaderComs {
    pub(super) vshader: ID3D11VertexShader,
    pub(super) pshader: ID3D11PixelShader,
    pub(super) ilayout: ID3D11InputLayout,
    pub(super) cbuffer: ID3D11Buffer,
}
impl ShaderComs {
    /// A function for create coms around shader effectively.
    pub(super) fn new(device: &ID3D11Device, dir: &String) -> Result<Self, WErr> {
        // Open vshader.cso here for input layout creation
        let mut vshader_bytebuf = Vec::new();
        File::open(dir.clone() + "vshader.cso")
            .map_err(|_| raise_err(EKnd::Io, "Open vshader.cso failed"))?
            .read_to_end(&mut vshader_bytebuf)
            .map_err(|_| raise_err(EKnd::Io, "Read vshader.cso failed"))?;
        let vshader_bytecode = vshader_bytebuf.as_ptr() as *const _ as *const ::core::ffi::c_void;
        // Vertex shader
        let vshader = unsafe {
            device
                .CreateVertexShader(vshader_bytecode, vshader_bytebuf.len(), None)
                .map_err(|_| raise_err(EKnd::Creation, "vshader"))?
        };
        // Pixel shader
        let pshader = unsafe {
            let mut buf = Vec::new();
            File::open(dir.clone() + "pshader.cso")
                .map_err(|_| raise_err(EKnd::Io, "Open pshader.cso failed"))?
                .read_to_end(&mut buf)
                .map_err(|_| raise_err(EKnd::Io, "Read pshader.cso failed"))?;
            let bytecode = buf.as_ptr() as *const _ as *const ::core::ffi::c_void;
            device
                .CreatePixelShader(bytecode, buf.len(), None)
                .map_err(|_| raise_err(EKnd::Creation, "pshader"))?
        };
        // Input layout
        let ilayout = unsafe {
            let pinputelementdescs = [
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PCSTR("POSITION\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PCSTR("COLOR\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: PCSTR("TEXCOORD\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];
            device
                .CreateInputLayout(
                    pinputelementdescs.as_ptr(),
                    pinputelementdescs.len() as u32,
                    vshader_bytecode,
                    vshader_bytebuf.len(),
                )
                .map_err(|_| raise_err(EKnd::Creation, "InputLayout"))?
        };
        // Constant buffer
        let cbuffer = unsafe {
            let cbuf_desc = D3D11_BUFFER_DESC {
                ByteWidth: size_of::<CData>() as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: 4,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };
            device
                .CreateBuffer(&cbuf_desc, std::ptr::null())
                .map_err(|_| raise_err(EKnd::Creation, "Cbuffer"))?
        };
        Ok(Self {
            vshader,
            pshader,
            ilayout,
            cbuffer,
        })
    }
}
