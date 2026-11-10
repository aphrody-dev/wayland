use crate::errors::{Dx12Error, Result};
use windows::core::Error;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxDrmFormat {
    Argb8888,
    Xrgb8888,
    Abgr8888,
    Xbgr8888,
    Nv12,
}

impl LinuxDrmFormat {
    pub fn to_dxgi_format(self) -> DXGI_FORMAT {
        match self {
            LinuxDrmFormat::Argb8888 => DXGI_FORMAT_B8G8R8A8_UNORM,
            LinuxDrmFormat::Xrgb8888 => DXGI_FORMAT_B8G8R8A8_UNORM,
            LinuxDrmFormat::Abgr8888 => DXGI_FORMAT_R8G8B8A8_UNORM,
            LinuxDrmFormat::Xbgr8888 => DXGI_FORMAT_R8G8B8A8_UNORM,
            LinuxDrmFormat::Nv12 => DXGI_FORMAT_NV12,
        }
    }

    pub fn from_fourcc(fourcc: u32) -> Option<Self> {
        // DRM FourCC standard definitions
        match fourcc {
            0x34325241 => Some(LinuxDrmFormat::Argb8888), // AR24
            0x34325258 => Some(LinuxDrmFormat::Xrgb8888), // XR24
            0x34324241 => Some(LinuxDrmFormat::Abgr8888), // AB24
            0x34324258 => Some(LinuxDrmFormat::Xbgr8888), // XB24
            0x3231564E => Some(LinuxDrmFormat::Nv12),     // NV12
            _ => None,
        }
    }
}

pub struct SharedGpuTexture {
    pub resource: ID3D12Resource,
    pub shared_handle: HANDLE,
    pub width: u32,
    pub height: u32,
    pub format: DXGI_FORMAT,
    pub size_bytes: u64,
}

impl SharedGpuTexture {
    pub fn create_shared(
        device: &ID3D12Device,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
    ) -> Result<Self> {
        unsafe {
            let heap_props = D3D12_HEAP_PROPERTIES {
                Type: D3D12_HEAP_TYPE_DEFAULT,
                CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
                MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
                CreationNodeMask: 0,
                VisibleNodeMask: 0,
            };

            let resource_desc = D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                Alignment: 0,
                Width: width as u64,
                Height: height,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: format,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
                Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET
                    | D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS,
            };

            let mut resource: Option<ID3D12Resource> = None;
            device.CreateCommittedResource(
                &heap_props,
                D3D12_HEAP_FLAG_SHARED,
                &resource_desc,
                D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut resource,
            )?;

            let resource = resource.ok_or_else(|| Dx12Error::SharedHandleFailed)?;

            let shared_handle = device.CreateSharedHandle(&resource, None, GENERIC_ALL.0, None)?;

            // Calculate size with row pitch alignment (typically 256 bytes per row in D3D12)
            let bpp = match format {
                DXGI_FORMAT_R8G8B8A8_UNORM | DXGI_FORMAT_B8G8R8A8_UNORM => 4,
                DXGI_FORMAT_NV12 => 1,
                _ => 4,
            };
            let row_pitch = (width * bpp + 255) & !255;
            let size_bytes = (row_pitch as u64) * (height as u64);

            Ok(Self {
                resource,
                shared_handle,
                width,
                height,
                format,
                size_bytes,
            })
        }
    }

    pub fn open_from_shared_handle(
        device: &ID3D12Device,
        shared_handle: HANDLE,
        width: u32,
        height: u32,
        format: DXGI_FORMAT,
    ) -> Result<Self> {
        unsafe {
            let mut resource: Option<ID3D12Resource> = None;
            device.OpenSharedHandle(shared_handle, &mut resource)?;
            let resource = resource.ok_or_else(|| Error::from_hresult(E_FAIL))?;

            let bpp = 4;
            let row_pitch = (width * bpp + 255) & !255;
            let size_bytes = (row_pitch as u64) * (height as u64);

            Ok(Self {
                resource,
                shared_handle,
                width,
                height,
                format,
                size_bytes,
            })
        }
    }
}

impl Drop for SharedGpuTexture {
    fn drop(&mut self) {
        if !self.shared_handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.shared_handle);
            }
        }
    }
}
