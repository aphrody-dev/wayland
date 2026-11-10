pub mod adapter;
pub mod device;
pub mod direct_composition;
pub mod errors;
pub mod fence;
pub mod queue;
pub mod shared_memory;

pub use adapter::{list_adapters, select_high_performance_adapter, AdapterInfo};
pub use device::{Dx12Context, Dx12UltimateFeatures};
pub use direct_composition::{DirectCompositionEngine, SwapChainConfig, WinUiDispatcherQueue};
pub use errors::{Dx12Error, Result};
pub use fence::GpuFence;
pub use queue::{CommandQueues, DescriptorHeaps};
pub use shared_memory::{LinuxDrmFormat, SharedGpuTexture};

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::Graphics::Dxgi::Common::*;

    #[test]
    fn test_rtx4070_hardware_initialization() {
        let (adapter, info) = select_high_performance_adapter().expect("Must find high performance GPU");
        println!("\n=== GPU Adapter Selected ===");
        println!("Adapter Name: {}", info.name);
        println!("Vendor ID: 0x{:04X} (NVIDIA: {})", info.vendor_id, info.is_nvidia);
        println!("Device ID: 0x{:04X}", info.device_id);
        println!("Dedicated VRAM: {:.2} MB ({:.2} GB)", info.vram_mb(), info.vram_gb());
        println!("Dedicated SysRAM: {:.2} MB", info.dedicated_system_memory as f64 / (1024.0 * 1024.0));
        println!("Shared SysRAM: {:.2} MB", info.shared_system_memory as f64 / (1024.0 * 1024.0));
        println!("Adapter LUID: Low=0x{:08X}, High=0x{:08X} (Total: 0x{:016X})", info.luid_low, info.luid_high, info.luid_u64());

        // Validate D3D12 device creation
        let ctx = Dx12Context::with_adapter(adapter, info.clone()).expect("D3D12 device creation failed");
        println!("D3D12 Feature Level: {}", ctx.ultimate_features.feature_level);
        println!("Raytracing Tier: {}", ctx.ultimate_features.raytracing_tier);
        println!("Mesh Shader Tier: {}", ctx.ultimate_features.mesh_shader_tier);
        println!("Variable Shading Rate Tier: {}", ctx.ultimate_features.variable_shading_rate_tier);
        println!("Sampler Feedback Tier: {}", ctx.ultimate_features.sampler_feedback_tier);
        println!("Wave Ops: {}, Lane Count: [{}..{}]", ctx.ultimate_features.wave_ops, ctx.ultimate_features.wave_lane_count_min, ctx.ultimate_features.wave_lane_count_max);

        // Command queues
        let queues = CommandQueues::new(&ctx.device).expect("Failed to create command queues");

        // Fence
        let mut fence = GpuFence::new(&ctx.device, 0).expect("Failed to create GPU fence");
        assert_eq!(fence.completed_value(), 0);

        // Signal fence on direct queue and wait on CPU
        let sig_val = fence.signal(&queues.direct_queue).expect("Failed to signal direct queue");
        fence.wait_cpu(sig_val, 2000).expect("GPU fence wait timeout");
        assert_eq!(fence.completed_value(), sig_val);

        // Shared NT Handle texture allocation for WSL2 DMA-BUF interop
        let width = 1920;
        let height = 1080;
        let shared_tex = SharedGpuTexture::create_shared(
            &ctx.device,
            width,
            height,
            DXGI_FORMAT_R8G8B8A8_UNORM,
        ).expect("Failed to create shared D3D12 texture");

        assert!(!shared_tex.shared_handle.is_invalid());
        println!("Allocated Shared GPU Texture: {}x{}, {} bytes, NT Handle: {:?}", width, height, shared_tex.size_bytes, shared_tex.shared_handle);

        // Verify OpenSharedHandle roundtrip
        let reopened = SharedGpuTexture::open_from_shared_handle(
            &ctx.device,
            shared_tex.shared_handle,
            width,
            height,
            DXGI_FORMAT_R8G8B8A8_UNORM,
        ).expect("Failed to open shared texture from NT handle");
        assert_eq!(reopened.width, width);
        assert_eq!(reopened.height, height);

        // DirectComposition
        let dcomp = DirectCompositionEngine::new().expect("Failed to initialize DirectComposition");
        let visual = dcomp.create_visual().expect("Failed to create DComp visual");
        let _ = visual;

        // WinUI Core DispatcherQueue
        let dispatcher = WinUiDispatcherQueue::new_dedicated_thread().expect("Failed to create WinUI DispatcherQueue");
        let _ = dispatcher;
    }
}
