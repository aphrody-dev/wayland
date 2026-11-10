use crate::errors::{Dx12Error, Result};
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::System::Threading::*;

pub struct GpuFence {
    pub fence: ID3D12Fence,
    pub event: HANDLE,
    pub current_value: u64,
}

impl GpuFence {
    pub fn new(device: &ID3D12Device, initial_value: u64) -> Result<Self> {
        unsafe {
            let fence: ID3D12Fence = device.CreateFence(initial_value, D3D12_FENCE_FLAG_NONE)?;
            let event = CreateEventW(None, false, false, None)?;
            Ok(Self {
                fence,
                event,
                current_value: initial_value,
            })
        }
    }

    pub fn signal(&mut self, queue: &ID3D12CommandQueue) -> Result<u64> {
        self.current_value += 1;
        unsafe {
            queue.Signal(&self.fence, self.current_value)?;
        }
        Ok(self.current_value)
    }

    pub fn wait_gpu(&self, queue: &ID3D12CommandQueue, value: u64) -> Result<()> {
        unsafe {
            queue.Wait(&self.fence, value)?;
        }
        Ok(())
    }

    pub fn wait_cpu(&self, value: u64, timeout_ms: u32) -> Result<()> {
        unsafe {
            if self.fence.GetCompletedValue() < value {
                self.fence.SetEventOnCompletion(value, self.event)?;
                let res = WaitForSingleObject(self.event, timeout_ms);
                if res != WAIT_OBJECT_0 {
                    return Err(Dx12Error::FenceTimeout(timeout_ms));
                }
            }
        }
        Ok(())
    }

    pub fn completed_value(&self) -> u64 {
        unsafe { self.fence.GetCompletedValue() }
    }
}

impl Drop for GpuFence {
    fn drop(&mut self) {
        if !self.event.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.event);
            }
        }
    }
}
