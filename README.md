# Sovereign Rust WSLg Compositor (`wslg-rs`): Direct3D 12 + Hyprland + COSMIC + Omarchy Patch

Native Rust 2024 WSLg compositor and protocol engine replacing legacy Weston/RDP WSLg with zero-copy Direct3D 12 Ultimate (`/dev/dxg` -> Windows NT Shared Handles), WinUI DirectComposition presentation, 120Hz Hyprland dynamic tiling, COSMIC desktop session compatibility, and Omarchy keyboard-first workflows.

---

## Workspace Architecture (`C:\src\crates\wayland`)

```
C:\src\crates\wayland\
├── Cargo.toml                      # Rust 2024 workspace
└── crates\
    ├── dx12-winui-core\            # D3D12 Ultimate device, GPU fences, NT shared textures, DirectComposition & WinUI DispatcherQueue
    ├── smithay-wayland-wire\       # Pure-Rust Wayland binary wire protocol, AF_HYPERV (hv_sock) transport, DMA-BUF & Fractional Scale globals
    ├── hypr-tiler\                 # Hyprland Dwindle BSP, Master-Stack, WindowRuleV2, 120Hz spring physics, Omarchy keybinds & /dev/dxg damage coalescer
    ├── cosmic-compat\              # COSMIC layer-shell, workspaces, Material 3 & Omarchy Aether RON theme bridge, HiDPI zero-blur protocol descriptors
    └── wslg-cli\                   # Executables: `wslg-rs.exe` and `wslg-cosmic-hypr.exe`
```

---

## Key Subsystems & Omarchy WSLg Patch

1. **Direct3D 12 Ultimate & Zero-Copy `/dev/dxg` Bridge (`dx12-winui-core` + `hypr-tiler::omarchy`)**:
   - Targets NVIDIA GeForce RTX 4070 (`0x10DE:0x2786`, Ada Lovelace `sm_89`, 12 GB VRAM, Feature Level `12_2`).
   - Allocates and imports cross-VM NT shared GPU textures (`DXGI_FORMAT_R8G8B8A8_UNORM`) paired with `DxgDmaBufDamageTracker`, coalescing Wayland surface damage regions into minimal bounding scissor rects for zero-copy `/dev/dxg` DMA-BUF presentation.
2. **Omarchy Keyboard-First Tiling Model (`hypr-tiler`)**:
   - Built-in `OmarchyKeybindRegistry` modeling Omarchy chords:
     - `Super+Space`: Omarchy root launcher (`omarchy-menu`)
     - `Super+K`: Keybinding HUD (`omarchy-menu-keybindings`)
     - `Super+Return`: GPU terminal (`ghostty`)
     - `Super+W`: Close active window
     - `Super+1..9` / `Super+Shift+1..9`: Direct workspace focus and window movement
     - `Super+Shift+T`: Live Aether theme hot-switch (`omarchy-wsl themes export <theme>`)
3. **COSMIC Theme Bridge & Zero-Blur HiDPI Scaling (`cosmic-compat` + `smithay-wayland-wire`)**:
   - `CosmicTheme::from_omarchy_palette(...)` maps Omarchy Aether palettes directly into COSMIC Desktop `.ron` theme structures.
   - Advertises 10 Wayland globals including `ext_workspace_manager_v1`, `zwp_linux_dmabuf_v1` (v4), `wp_fractional_scale_manager_v1` (v1), and `wp_viewporter` (v1).
   - `WslgHiDpiScalingDescriptor` computes exact 120ths fractional scale factors from Windows 11 monitor DPI (e.g., 144 DPI = `180/120` = 1.5x, mapping `2560x1440` logical coordinates cleanly onto `3840x2160` physical buffers without Weston/XWayland bilinear blur).

---

## CLI Usage (`wslg-rs`)

Installed in `C:\Users\aphro\.local\bin\wslg-rs.exe`:

```powershell
# Inspect GPU adapter, D3D12 Ultimate tiers, DirectComposition, and Wayland globals
wslg-rs info

# Validate the full Hyprland + COSMIC + Omarchy WSLg patch with live D3D12 + Wayland wire checks
wslg-rs omarchy-patch --theme tokyo-night
wslg-rs omarchy-patch --theme cachyos-emerald --json

# Benchmark D3D12 GPU fence latency and NT shared handle roundtrips
wslg-rs benchmark-dx12 --iterations 100

# Simulate a 120Hz Wayland + COSMIC + Hyprland Dwindle session
wslg-rs simulate-session
```

---

## Verification

```powershell
cargo test --workspace --manifest-path C:\src\crates\wayland\Cargo.toml
```
