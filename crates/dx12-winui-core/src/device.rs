use crate::adapter::{select_high_performance_adapter, AdapterInfo};
use crate::errors::{Dx12Error, Result};
use tracing::info;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::IDXGIAdapter1;

#[derive(Debug, Clone)]
pub struct Dx12UltimateFeatures {
    pub feature_level: String,
    pub raytracing_tier: u32,
    pub mesh_shader_tier: u32,
    pub variable_shading_rate_tier: u32,
    pub sampler_feedback_tier: u32,
    pub wave_ops: bool,
    pub wave_lane_count_min: u32,
    pub wave_lane_count_max: u32,
}

pub struct Dx12Context {
    pub adapter: IDXGIAdapter1,
    pub adapter_info: AdapterInfo,
    pub device: ID3D12Device,
    pub ultimate_features: Dx12UltimateFeatures,
}

impl Dx12Context {
    pub fn new() -> Result<Self> {
        let (adapter, adapter_info) = select_high_performance_adapter()?;
        Self::with_adapter(adapter, adapter_info)
    }

    pub fn with_adapter(adapter: IDXGIAdapter1, adapter_info: AdapterInfo) -> Result<Self> {
        let (device, feature_level_str) = unsafe {
            let mut dev: Option<ID3D12Device> = None;
            // Attempt Feature Level 12_2 first (DirectX 12 Ultimate)
            let res = D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_12_2, &mut dev);
            if res.is_ok() && dev.is_some() {
                (dev.unwrap(), "12_2 (DirectX 12 Ultimate)")
            } else {
                let mut dev_fallback: Option<ID3D12Device> = None;
                D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_12_1, &mut dev_fallback)?;
                (
                    dev_fallback.ok_or_else(|| {
                        Dx12Error::FeatureLevelNotSupported("12_1".into())
                    })?,
                    "12_1 (DirectX 12_1)",
                )
            }
        };

        let ultimate_features = unsafe {
            // Check Raytracing
            let mut d3d12_options5 = D3D12_FEATURE_DATA_D3D12_OPTIONS5::default();
            let _ = device.CheckFeatureSupport(
                D3D12_FEATURE_D3D12_OPTIONS5,
                &mut d3d12_options5 as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS5>() as u32,
            );

            // Check Mesh Shaders and Sampler Feedback
            let mut d3d12_options7 = D3D12_FEATURE_DATA_D3D12_OPTIONS7::default();
            let _ = device.CheckFeatureSupport(
                D3D12_FEATURE_D3D12_OPTIONS7,
                &mut d3d12_options7 as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS7>() as u32,
            );

            // Check Variable Shading Rate
            let mut d3d12_options6 = D3D12_FEATURE_DATA_D3D12_OPTIONS6::default();
            let _ = device.CheckFeatureSupport(
                D3D12_FEATURE_D3D12_OPTIONS6,
                &mut d3d12_options6 as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS6>() as u32,
            );

            // Check Wave Ops
            let mut d3d12_wave = D3D12_FEATURE_DATA_D3D12_OPTIONS1::default();
            let _ = device.CheckFeatureSupport(
                D3D12_FEATURE_D3D12_OPTIONS1,
                &mut d3d12_wave as *mut _ as *mut _,
                std::mem::size_of::<D3D12_FEATURE_DATA_D3D12_OPTIONS1>() as u32,
            );

            Dx12UltimateFeatures {
                feature_level: feature_level_str.to_string(),
                raytracing_tier: d3d12_options5.RaytracingTier.0 as u32,
                mesh_shader_tier: d3d12_options7.MeshShaderTier.0 as u32,
                variable_shading_rate_tier: d3d12_options6.VariableShadingRateTier.0 as u32,
                sampler_feedback_tier: d3d12_options7.SamplerFeedbackTier.0 as u32,
                wave_ops: d3d12_wave.WaveOps.as_bool(),
                wave_lane_count_min: d3d12_wave.WaveLaneCountMin,
                wave_lane_count_max: d3d12_wave.WaveLaneCountMax,
            }
        };

        info!(
            "D3D12 Device initialized on {} [{}] (Raytracing Tier {}, Mesh Shader Tier {})",
            adapter_info.name,
            ultimate_features.feature_level,
            ultimate_features.raytracing_tier,
            ultimate_features.mesh_shader_tier
        );

        Ok(Self {
            adapter,
            adapter_info,
            device,
            ultimate_features,
        })
    }
}
