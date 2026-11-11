// Wayland Core Protocol Global Interface Names
pub const INTERFACE_WL_DISPLAY: &str = "wl_display";
pub const INTERFACE_WL_REGISTRY: &str = "wl_registry";
pub const INTERFACE_WL_CALLBACK: &str = "wl_callback";
pub const INTERFACE_WL_COMPOSITOR: &str = "wl_compositor";
pub const INTERFACE_WL_SHM: &str = "wl_shm";
pub const INTERFACE_WL_SHM_POOL: &str = "wl_shm_pool";
pub const INTERFACE_WL_BUFFER: &str = "wl_buffer";
pub const INTERFACE_WL_SURFACE: &str = "wl_surface";
pub const INTERFACE_WL_SUBCOMPOSITOR: &str = "wl_subcompositor";
pub const INTERFACE_WL_SUBSURFACE: &str = "wl_subsurface";
pub const INTERFACE_WL_SEAT: &str = "wl_seat";
pub const INTERFACE_WL_POINTER: &str = "wl_pointer";
pub const INTERFACE_WL_KEYBOARD: &str = "wl_keyboard";
pub const INTERFACE_WL_TOUCH: &str = "wl_touch";
pub const INTERFACE_WL_OUTPUT: &str = "wl_output";

// XDG Shell Protocol
pub const INTERFACE_XDG_WM_BASE: &str = "xdg_wm_base";
pub const INTERFACE_XDG_SURFACE: &str = "xdg_surface";
pub const INTERFACE_XDG_TOPLEVEL: &str = "xdg_toplevel";
pub const INTERFACE_XDG_POPUP: &str = "xdg_popup";

// Viewporter & Fractional Scaling Protocols
pub const INTERFACE_WP_VIEWPORTER: &str = "wp_viewporter";
pub const INTERFACE_WP_VIEWPORT: &str = "wp_viewport";
pub const INTERFACE_WP_FRACTIONAL_SCALE_MANAGER_V1: &str = "wp_fractional_scale_manager_v1";
pub const INTERFACE_WP_FRACTIONAL_SCALE_V1: &str = "wp_fractional_scale_v1";

// Linux DMA-BUF Protocol (mapping Linux /dev/dxg to D3D12 NT Shared Handles)
pub const INTERFACE_ZWP_LINUX_DMABUF_V1: &str = "zwp_linux_dmabuf_v1";
pub const INTERFACE_ZWP_LINUX_BUFFER_PARAMS_V1: &str = "zwp_linux_buffer_params_v1";

// Ext Workspace Management Protocol (COSMIC + Hyprland + Omarchy bar workspace sync)
pub const INTERFACE_EXT_WORKSPACE_MANAGER_V1: &str = "ext_workspace_manager_v1";
pub const INTERFACE_EXT_WORKSPACE_GROUP_HANDLE_V1: &str = "ext_workspace_group_handle_v1";
pub const INTERFACE_EXT_WORKSPACE_HANDLE_V1: &str = "ext_workspace_handle_v1";

// Opcodes: wl_display
pub mod display {
    pub const REQ_SYNC: u16 = 0;
    pub const REQ_GET_REGISTRY: u16 = 1;
    pub const EVT_ERROR: u16 = 0;
    pub const EVT_DELETE_ID: u16 = 1;
}

// Opcodes: wl_registry
pub mod registry {
    pub const REQ_BIND: u16 = 0;
    pub const EVT_GLOBAL: u16 = 0;
    pub const EVT_GLOBAL_REMOVE: u16 = 1;
}

// Opcodes: wl_compositor
pub mod compositor {
    pub const REQ_CREATE_SURFACE: u16 = 0;
    pub const REQ_CREATE_REGION: u16 = 1;
}

// Opcodes: wl_surface
pub mod surface {
    pub const REQ_DESTROY: u16 = 0;
    pub const REQ_ATTACH: u16 = 1;
    pub const REQ_DAMAGE: u16 = 2;
    pub const REQ_FRAME: u16 = 3;
    pub const REQ_SET_OPAQUE_REGION: u16 = 4;
    pub const REQ_SET_INPUT_REGION: u16 = 5;
    pub const REQ_COMMIT: u16 = 6;
    pub const REQ_SET_BUFFER_TRANSFORM: u16 = 7;
    pub const REQ_SET_BUFFER_SCALE: u16 = 8;
    pub const REQ_DAMAGE_BUFFER: u16 = 9;
    pub const REQ_OFFSET: u16 = 10;
}

// Opcodes: xdg_wm_base
pub mod xdg_wm_base {
    pub const REQ_DESTROY: u16 = 0;
    pub const REQ_CREATE_POSITIONER: u16 = 1;
    pub const REQ_GET_XDG_SURFACE: u16 = 2;
    pub const REQ_PONG: u16 = 3;
    pub const EVT_PING: u16 = 0;
}

// Opcodes: xdg_surface
pub mod xdg_surface {
    pub const REQ_DESTROY: u16 = 0;
    pub const REQ_GET_TOPLEVEL: u16 = 1;
    pub const REQ_GET_POPUP: u16 = 2;
    pub const REQ_SET_WINDOW_GEOMETRY: u16 = 3;
    pub const REQ_ACK_CONFIGURE: u16 = 4;
    pub const EVT_CONFIGURE: u16 = 0;
}

// Opcodes: xdg_toplevel
pub mod xdg_toplevel {
    pub const REQ_DESTROY: u16 = 0;
    pub const REQ_SET_PARENT: u16 = 1;
    pub const REQ_SET_TITLE: u16 = 2;
    pub const REQ_SET_APP_ID: u16 = 3;
    pub const REQ_SHOW_WINDOW_MENU: u16 = 4;
    pub const REQ_MOVE: u16 = 5;
    pub const REQ_RESIZE: u16 = 6;
    pub const REQ_SET_MAX_SIZE: u16 = 7;
    pub const REQ_SET_MIN_SIZE: u16 = 8;
    pub const REQ_SET_MAXIMIZED: u16 = 9;
    pub const REQ_UNSET_MAXIMIZED: u16 = 10;
    pub const REQ_SET_FULLSCREEN: u16 = 11;
    pub const REQ_UNSET_FULLSCREEN: u16 = 12;
    pub const REQ_SET_MINIMIZED: u16 = 13;
    pub const EVT_CONFIGURE: u16 = 0;
    pub const EVT_CLOSE: u16 = 1;
}

// Opcodes: zwp_linux_dmabuf_v1
pub mod linux_dmabuf {
    pub const REQ_DESTROY: u16 = 0;
    pub const REQ_CREATE_PARAMS: u16 = 1;
    pub const REQ_GET_DEFAULT_FEEDBACK: u16 = 2;
    pub const REQ_GET_SURFACE_FEEDBACK: u16 = 3;
    pub const EVT_FORMAT: u16 = 0;
    pub const EVT_MODIFIER: u16 = 1;
}

// Opcodes: wl_output
pub mod output {
    pub const EVT_GEOMETRY: u16 = 0;
    pub const EVT_MODE: u16 = 1;
    pub const EVT_DONE: u16 = 2;
    pub const EVT_SCALE: u16 = 3;
    pub const EVT_NAME: u16 = 4;
    pub const EVT_DESCRIPTION: u16 = 5;
}

// Opcodes: wp_fractional_scale_v1
pub mod fractional_scale {
    pub const EVT_PREFERRED_SCALE: u16 = 0;
}
