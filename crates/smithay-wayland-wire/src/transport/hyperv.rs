use windows::core::GUID;

// AF_HYPERV socket domain ID in Windows Sockets 2
pub const AF_HYPERV: u16 = 34;

// Well-known Hyper-V VMADDR constants for WSL2
pub const HV_GUID_ZERO: GUID = GUID::zeroed();
pub const HV_GUID_WILDCARD: GUID = GUID::zeroed();
pub const HV_GUID_BROADCAST: GUID = GUID::from_u128(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
pub const HV_GUID_CHILDREN: GUID = GUID::from_u128(0x90db3b86_dd8d_4b25_8336_6c63b7e8d123);
pub const HV_GUID_LOOPBACK: GUID = GUID::from_u128(0xe0e45303_f35f_4ac8_9a60_22430b0ebebf);
pub const HV_GUID_PARENT: GUID = GUID::from_u128(0xa42e7cda_d03f_480c_9cc2_a4de20abb878);

// WSL2 Linux guest vsock bridge uses port 0x40000000 + vsock_port
// for Wayland display socket (usually WAYLAND_DISPLAY=wayland-0 -> port 1000 / 0x400003e8)
pub const WSL2_WAYLAND_VSOCK_PORT: u32 = 0x400003E8;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrHv {
    pub family: u16,
    pub reserved: u16,
    pub vm_id: GUID,
    pub service_id: GUID,
}

impl SockAddrHv {
    pub fn new_wsl2_wayland_server() -> Self {
        // Construct ServiceId with WSL2 vsock port template
        let service_id = GUID::from_values(
            WSL2_WAYLAND_VSOCK_PORT,
            0xFACB,
            0x11E6,
            [0xBD, 0x58, 0x64, 0x00, 0x6A, 0x79, 0x86, 0xF2],
        );

        Self {
            family: AF_HYPERV,
            reserved: 0,
            vm_id: HV_GUID_WILDCARD, // accept connections from any WSL2 VM container
            service_id,
        }
    }
}
