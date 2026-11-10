use crate::errors::{Dx12Error, Result};
use tracing::{debug, info};
use windows::Win32::Graphics::Dxgi::*;

#[derive(Debug, Clone)]
pub struct AdapterInfo {
    pub index: u32,
    pub name: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub luid_low: u32,
    pub luid_high: i32,
    pub is_nvidia: bool,
    pub is_hardware: bool,
}

impl AdapterInfo {
    pub fn vram_mb(&self) -> f64 {
        self.dedicated_video_memory as f64 / (1024.0 * 1024.0)
    }

    pub fn vram_gb(&self) -> f64 {
        self.dedicated_video_memory as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    pub fn luid_u64(&self) -> u64 {
        ((self.luid_high as u64) << 32) | (self.luid_low as u64)
    }
}

pub fn list_adapters() -> Result<Vec<(IDXGIAdapter1, AdapterInfo)>> {
    let mut adapters = Vec::new();
    unsafe {
        let factory: IDXGIFactory6 = CreateDXGIFactory2(DXGI_CREATE_FACTORY_FLAGS(0))?;
        let mut index = 0;
        loop {
            let res = factory.EnumAdapterByGpuPreference::<IDXGIAdapter1>(
                index,
                DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
            );
            match res {
                Ok(adapter) => {
                    let desc = adapter.GetDesc1()?;
                    let name = String::from_utf16_lossy(
                        &desc.Description[..desc
                            .Description
                            .iter()
                            .position(|&c| c == 0)
                            .unwrap_or(desc.Description.len())],
                    );

                    let is_software = (desc.Flags & (DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32)) != 0;
                    let is_nvidia = desc.VendorId == 0x10DE;

                    let info = AdapterInfo {
                        index,
                        name,
                        vendor_id: desc.VendorId,
                        device_id: desc.DeviceId,
                        sub_sys_id: desc.SubSysId,
                        revision: desc.Revision,
                        dedicated_video_memory: desc.DedicatedVideoMemory,
                        dedicated_system_memory: desc.DedicatedSystemMemory,
                        shared_system_memory: desc.SharedSystemMemory,
                        luid_low: desc.AdapterLuid.LowPart,
                        luid_high: desc.AdapterLuid.HighPart,
                        is_nvidia,
                        is_hardware: !is_software,
                    };
                    debug!("Found adapter [{}]: {} ({:.2} GB VRAM)", index, info.name, info.vram_gb());
                    adapters.push((adapter, info));
                    index += 1;
                }
                Err(_) => break,
            }
        }
    }

    if adapters.is_empty() {
        return Err(Dx12Error::NoAdapterFound);
    }

    Ok(adapters)
}

pub fn select_high_performance_adapter() -> Result<(IDXGIAdapter1, AdapterInfo)> {
    let mut all = list_adapters()?;
    // Prefer NVIDIA RTX hardware adapter first (e.g. GeForce RTX 4070)
    if let Some(pos) = all.iter().position(|(_, info)| info.is_hardware && info.is_nvidia) {
        let selected = all.swap_remove(pos);
        info!("Selected NVIDIA GPU: {} ({:.2} GB VRAM)", selected.1.name, selected.1.vram_gb());
        return Ok(selected);
    }

    // Otherwise first hardware adapter
    if let Some(pos) = all.iter().position(|(_, info)| info.is_hardware) {
        let selected = all.swap_remove(pos);
        info!("Selected Hardware GPU: {} ({:.2} GB VRAM)", selected.1.name, selected.1.vram_gb());
        return Ok(selected);
    }

    Ok(all.swap_remove(0))
}
