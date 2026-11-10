use crate::errors::{Dx12Error, Result};
use tracing::info;
use windows::System::DispatcherQueueController;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::DirectComposition::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DispatcherQueueOptions, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
};

pub struct WinUiDispatcherQueue {
    pub controller: DispatcherQueueController,
}

impl WinUiDispatcherQueue {
    pub fn new_on_current_thread() -> Result<Self> {
        let options = DispatcherQueueOptions {
            dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
            threadType: DQTYPE_THREAD_CURRENT,
            apartmentType: DQTAT_COM_NONE,
        };
        unsafe {
            let controller = CreateDispatcherQueueController(options)
                .map_err(|_| Dx12Error::DispatcherQueueFailed)?;
            Ok(Self { controller })
        }
    }

    pub fn new_dedicated_thread() -> Result<Self> {
        let controller = DispatcherQueueController::CreateOnDedicatedThread()
            .map_err(|e| Dx12Error::WindowsError(e))?;
        info!("WinUI Core DispatcherQueueController created on dedicated thread");
        Ok(Self { controller })
    }
}

pub struct DirectCompositionEngine {
    pub dcomp_device: IDCompositionDevice,
}

impl DirectCompositionEngine {
    pub fn new() -> Result<Self> {
        unsafe {
            let dcomp_device: IDCompositionDevice = DCompositionCreateDevice(None)
                .map_err(|_| Dx12Error::DirectCompositionFailed)?;
            info!("DirectComposition device created successfully");
            Ok(Self { dcomp_device })
        }
    }

    pub fn create_visual(&self) -> Result<IDCompositionVisual> {
        unsafe {
            let visual = self.dcomp_device.CreateVisual()?;
            Ok(visual)
        }
    }

    pub fn create_target_for_hwnd(&self, hwnd: HWND, topmost: bool) -> Result<IDCompositionTarget> {
        unsafe {
            let target = self.dcomp_device.CreateTargetForHwnd(hwnd, topmost)?;
            Ok(target)
        }
    }

    pub fn commit(&self) -> Result<()> {
        unsafe {
            self.dcomp_device.Commit()?;
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SwapChainConfig {
    pub width: u32,
    pub height: u32,
    pub buffer_count: u32,
    pub format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub swap_effect: DXGI_SWAP_EFFECT,
    pub flags: DXGI_SWAP_CHAIN_FLAG,
}

impl Default for SwapChainConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            buffer_count: 2,
            format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
            swap_effect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING,
        }
    }
}
