use dx12_winui_core::LinuxDrmFormat;
use std::collections::HashMap;
use windows::Win32::Foundation::HANDLE;

#[derive(Debug, Clone)]
pub struct DmaBufPlane {
    pub fd: i32,
    pub plane_index: u32,
    pub offset: u32,
    pub stride: u32,
    pub modifier: u64,
}

#[derive(Debug, Clone)]
pub struct DmaBufImport {
    pub buffer_id: u32,
    pub width: u32,
    pub height: u32,
    pub format_fourcc: u32,
    pub drm_format: Option<LinuxDrmFormat>,
    pub modifier: u64,
    pub planes: Vec<DmaBufPlane>,
    pub nt_shared_handle: Option<usize>, // Raw Windows NT HANDLE
}

impl DmaBufImport {
    pub fn is_supported(&self) -> bool {
        self.drm_format.is_some()
    }
}

pub struct D3D12SharedMappingTable {
    pub mappings: HashMap<u32, DmaBufImport>,
}

impl D3D12SharedMappingTable {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn register_dmabuf(
        &mut self,
        buffer_id: u32,
        width: u32,
        height: u32,
        format_fourcc: u32,
        modifier: u64,
        planes: Vec<DmaBufPlane>,
        nt_handle: Option<HANDLE>,
    ) {
        let nt_shared_handle = nt_handle.map(|h| h.0 as usize);
        let drm_format = LinuxDrmFormat::from_fourcc(format_fourcc);
        self.mappings.insert(
            buffer_id,
            DmaBufImport {
                buffer_id,
                width,
                height,
                format_fourcc,
                drm_format,
                modifier,
                planes,
                nt_shared_handle,
            },
        );
    }

    pub fn get(&self, buffer_id: u32) -> Option<&DmaBufImport> {
        self.mappings.get(&buffer_id)
    }

    pub fn remove(&mut self, buffer_id: u32) -> Option<DmaBufImport> {
        self.mappings.remove(&buffer_id)
    }
}
