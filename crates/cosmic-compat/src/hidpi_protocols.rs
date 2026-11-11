use serde::{Deserialize, Serialize};
use smithay_wayland_wire::proto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WslgProtocolDescriptor {
    pub interface: String,
    pub version: u32,
    pub role: String,
    pub zero_copy_or_hidpi: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WslgHiDpiScalingDescriptor {
    pub windows_dpi: u32,
    pub scale_factor_120ths: u32,
    pub logical_width: u32,
    pub logical_height: u32,
    pub physical_buffer_width: u32,
    pub physical_buffer_height: u32,
    pub zero_blur_pixel_exact: bool,
    pub protocols: Vec<WslgProtocolDescriptor>,
}

impl WslgHiDpiScalingDescriptor {
    /// Computes pixel-exact `wp_fractional_scale_v1` (in 120ths) + `wp_viewporter` dimensions
    /// from Windows 11 monitor DPI (96 DPI = 1.0x = 120, 120 DPI = 1.25x = 150, 144 DPI = 1.5x = 180, 192 DPI = 2.0x = 240),
    /// eliminating classic Weston/RDP bilinear scaling blur on Windows 11 WSLg.
    pub fn from_windows_dpi(physical_width: u32, physical_height: u32, windows_dpi: u32) -> Self {
        let clamped_dpi = windows_dpi.max(96);
        // 120ths = windows_dpi * 120 / 96 = windows_dpi * 5 / 4
        let scale_120ths = (clamped_dpi * 5) / 4;
        let logical_width = (physical_width * 120) / scale_120ths;
        let logical_height = (physical_height * 120) / scale_120ths;

        Self {
            windows_dpi: clamped_dpi,
            scale_factor_120ths: scale_120ths,
            logical_width,
            logical_height,
            physical_buffer_width: physical_width,
            physical_buffer_height: physical_height,
            zero_blur_pixel_exact: true,
            protocols: Self::omarchy_wslg_protocols(),
        }
    }

    pub fn omarchy_wslg_protocols() -> Vec<WslgProtocolDescriptor> {
        vec![
            WslgProtocolDescriptor {
                interface: proto::INTERFACE_EXT_WORKSPACE_MANAGER_V1.to_string(),
                version: 1,
                role: "Unified COSMIC + Hyprland + Omarchy bar workspace state synchronization"
                    .to_string(),
                zero_copy_or_hidpi: false,
            },
            WslgProtocolDescriptor {
                interface: proto::INTERFACE_ZWP_LINUX_DMABUF_V1.to_string(),
                version: 4,
                role: "Zero-copy /dev/dxg DMA-BUF import to Windows D3D12 NT shared texture handles"
                    .to_string(),
                zero_copy_or_hidpi: true,
            },
            WslgProtocolDescriptor {
                interface: proto::INTERFACE_WP_FRACTIONAL_SCALE_V1.to_string(),
                version: 1,
                role: "Exact 120ths fractional DPI scale factor negotiation matching Windows 11 monitor DPI"
                    .to_string(),
                zero_copy_or_hidpi: true,
            },
            WslgProtocolDescriptor {
                interface: proto::INTERFACE_WP_VIEWPORTER.to_string(),
                version: 1,
                role: "1:1 physical pixel buffer viewport mapping bypassing Weston/RDP bilinear blur"
                    .to_string(),
                zero_copy_or_hidpi: true,
            },
        ]
    }
}
