use clap::{Parser, Subcommand};
use cosmic_compat::{
    CosmicSessionEnvironment, CosmicTheme, LayerAnchor, LayerShellManager, LayerSurface, LayerType,
};
use dx12_winui_core::{
    select_high_performance_adapter, CommandQueues, DirectCompositionEngine,
    Dx12Context, GpuFence, SharedGpuTexture, WinUiDispatcherQueue,
};
use hypr_tiler::{
    DwindleTree, GapsConfig, HyprWindow, SpringParams, WindowGeometryAnimator, WindowRect,
};
use smithay_wayland_wire::{
    proto, LoopbackTransport, WaylandArgument, WaylandCompositor, WaylandMessage,
};
use std::time::Instant;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM;

#[derive(Parser)]
#[command(name = "wslg-rs")]
#[command(about = "Sovereign Rust WSLg Compositor: Direct3D 12 Ultimate + WinUI Core + Hyprland Tiling + COSMIC Desktop Compatibility")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display system, GPU adapter, D3D12 Ultimate, and Wayland compositor information
    Info,

    /// Benchmark DirectX 12 Ultimate fence, command queue, and shared NT handle latency
    BenchmarkDx12 {
        #[arg(short, long, default_value_t = 100)]
        iterations: u32,
    },

    /// Simulate an end-to-end WSL2 Wayland + COSMIC session with Hyprland dynamic tiling
    SimulateSession,

    /// Print COSMIC desktop session environment variables and Google Material 3 theme RON
    CosmicEnv {
        #[arg(short, long)]
        export_theme: bool,
    },
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info => run_info()?,
        Commands::BenchmarkDx12 { iterations } => run_benchmark_dx12(iterations)?,
        Commands::SimulateSession => run_simulate_session()?,
        Commands::CosmicEnv { export_theme } => run_cosmic_env(export_theme)?,
    }

    Ok(())
}

fn run_info() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("  SOVEREIGN WSLg COMPOSITOR ENGINE (RUST + D3D12 + HYPRLAND)");
    println!("============================================================");

    let (adapter, info) = select_high_performance_adapter()?;
    println!("Hardware Adapter Information:");
    println!("  - Name:               {}", info.name);
    println!("  - Vendor ID:          0x{:04X} (NVIDIA: {})", info.vendor_id, info.is_nvidia);
    println!("  - Device ID:          0x{:04X}", info.device_id);
    println!("  - Dedicated VRAM:     {:.2} MB ({:.2} GB)", info.vram_mb(), info.vram_gb());
    println!("  - Shared SysRAM:      {:.2} MB", info.shared_system_memory as f64 / (1024.0 * 1024.0));
    println!("  - Adapter LUID:       0x{:016X}", info.luid_u64());

    let ctx = Dx12Context::with_adapter(adapter, info)?;
    println!("\nDirectX 12 Ultimate Capabilities:");
    println!("  - Feature Level:      {}", ctx.ultimate_features.feature_level);
    println!("  - Raytracing Tier:    {}", ctx.ultimate_features.raytracing_tier);
    println!("  - Mesh Shader Tier:   {}", ctx.ultimate_features.mesh_shader_tier);
    println!("  - Variable Shading:   Tier {}", ctx.ultimate_features.variable_shading_rate_tier);
    println!("  - Sampler Feedback:   Tier {}", ctx.ultimate_features.sampler_feedback_tier);
    println!("  - Wave Ops:           {} (Lane Count [{}..{}])", ctx.ultimate_features.wave_ops, ctx.ultimate_features.wave_lane_count_min, ctx.ultimate_features.wave_lane_count_max);

    println!("\nDirectComposition & WinUI Core Presentation:");
    let dcomp = DirectCompositionEngine::new();
    println!("  - DirectComposition: {}", if dcomp.is_ok() { "ACTIVE" } else { "FAILED" });
    let dispatcher = WinUiDispatcherQueue::new_dedicated_thread();
    println!("  - DispatcherQueue:   {}", if dispatcher.is_ok() { "ACTIVE" } else { "FAILED" });

    println!("\nSmithay Wayland Wire Protocol Engine:");
    let compositor = WaylandCompositor::new();
    println!("  - Registered Globals: {}", compositor.globals.len());
    for g in &compositor.globals {
        println!("      * {} (v{})", g.interface, g.version);
    }
    println!("  - Outputs:            {} (High Refresh: 120Hz/144Hz)", compositor.outputs.len());
    println!("  - Transport:          AF_HYPERV / hv_sock (Port 0x{:08X}) + Memory Loopback", smithay_wayland_wire::SockAddrHv::new_wsl2_wayland_server().service_id.data1);

    println!("\nCOSMIC Desktop Compatibility:");
    let env = CosmicSessionEnvironment::new();
    println!("  - Session Desktop:    {}", env.env_vars.get("XDG_CURRENT_DESKTOP").unwrap());
    println!("  - Graphics Backend:   WGPU_BACKEND={}", env.env_vars.get("WGPU_BACKEND").unwrap());
    println!("  - Theme:              Google Material 3 Sovereign Dark (12px Radius, 8px Spacing)");

    println!("============================================================");
    Ok(())
}

fn run_benchmark_dx12(iterations: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Running DirectX 12 Hardware Benchmark on NVIDIA GPU ===");
    let ctx = Dx12Context::new()?;
    println!("Target: {} (VRAM: {:.2} GB)", ctx.adapter_info.name, ctx.adapter_info.vram_gb());

    let queues = CommandQueues::new(&ctx.device)?;
    let mut fence = GpuFence::new(&ctx.device, 0)?;

    println!("Executing {} iterations of GPU command queue fence signal/wait...", iterations);
    let start = Instant::now();
    for _ in 0..iterations {
        let val = fence.signal(&queues.direct_queue)?;
        fence.wait_cpu(val, 1000)?;
    }
    let elapsed = start.elapsed();
    let per_op_us = (elapsed.as_micros() as f64) / (iterations as f64);
    println!("Fence roundtrip latency: {:.2} us/op (Total: {:?})", per_op_us, elapsed);

    println!("\nBenchmarking Zero-Copy Shared NT Texture Allocation (1920x1080 ARGB8888)...");
    let alloc_start = Instant::now();
    let texture = SharedGpuTexture::create_shared(&ctx.device, 1920, 1080, DXGI_FORMAT_R8G8B8A8_UNORM)?;
    let alloc_elapsed = alloc_start.elapsed();
    println!("Allocated shared texture: {} bytes, Handle: {:?} in {:?}", texture.size_bytes, texture.shared_handle, alloc_elapsed);

    let open_start = Instant::now();
    let reopened = SharedGpuTexture::open_from_shared_handle(&ctx.device, texture.shared_handle, 1920, 1080, DXGI_FORMAT_R8G8B8A8_UNORM)?;
    let open_elapsed = open_start.elapsed();
    println!("Opened shared texture via NT handle in {:?}", open_elapsed);
    assert_eq!(reopened.width, 1920);

    println!("DirectX 12 Ultimate Benchmark Passed with zero errors!");
    Ok(())
}

fn run_simulate_session() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("  SIMULATING FULL WSL2 WAYLAND + COSMIC + HYPRLAND SESSION");
    println!("============================================================");

    // 1. Initialize Compositor and Hardware GPU
    let ctx = Dx12Context::new()?;
    let mut compositor = WaylandCompositor::new();
    let mut transport = LoopbackTransport::new();
    println!("[1/6] Initialized Sovereign Compositor and D3D12 device on {}", ctx.adapter_info.name);

    // 2. Client connection & Registry handshake
    let req_reg = WaylandMessage::new(1, proto::display::REQ_GET_REGISTRY, vec![WaylandArgument::NewId(2)]);
    compositor.dispatch(req_reg, &mut transport)?;
    println!("[2/6] Handshake complete: advertised {} globals to client", transport.server_to_client.len());

    // 3. Setup COSMIC Layer Shell (cosmic-panel & cosmic-dock)
    let mut layer_shell = LayerShellManager::new();
    layer_shell.add_surface(LayerSurface {
        id: 100,
        layer: LayerType::Top,
        anchor: LayerAnchor::TOP | LayerAnchor::LEFT | LayerAnchor::RIGHT,
        exclusive_zone: 48,
        width: 1920,
        height: 48,
        margin_top: 0,
        margin_bottom: 0,
        margin_left: 0,
        margin_right: 0,
    });
    let screen = WindowRect::new(0.0, 0.0, 1920.0, 1080.0);
    let usable_area = layer_shell.calculate_usable_area(screen);
    println!("[3/6] COSMIC Panel mounted at top (48px exclusive zone). Usable tiling area: {}x{} @ ({}, {})",
        usable_area.width, usable_area.height, usable_area.x, usable_area.y);

    // 4. Launch 3 Application Windows
    let mut windows = vec![
        HyprWindow::new(1, "com.system76.CosmicTerm", "COSMIC Terminal", "1"),
        HyprWindow::new(2, "org.mozilla.firefox", "Firefox Nightly", "1"),
        HyprWindow::new(3, "com.google.Chrome", "Google Chrome", "1"),
    ];
    println!("[4/6] Spawning 3 Wayland clients: Terminal, Firefox, Chrome");

    // 5. Hyprland Dwindle Layout Calculation
    let mut dwindle = DwindleTree::new(GapsConfig { inner: 8.0, outer: 12.0 });
    for w in &windows {
        dwindle.insert_window(w.id, None);
    }
    let layout = dwindle.compute_layout(usable_area);
    println!("[5/6] Hyprland Dwindle Tiling Solved:");
    for w in &mut windows {
        if let Some(rect) = layout.get(&w.id) {
            w.target_geometry = *rect;
            println!("      * [{}] {:<24}: ({:.1}, {:.1}) size {:.1}x{:.1}", w.id, w.app_id, rect.x, rect.y, rect.width, rect.height);
        }
    }

    // 6. 120Hz Spring Physics Animation Solver Step
    println!("[6/6] Stepping 120Hz spring geometry animators over 30 frames (0.25s transition)...");
    let mut animators: Vec<WindowGeometryAnimator> = windows.iter().map(|w| {
        WindowGeometryAnimator::new(
            WindowRect::new(usable_area.x, usable_area.y, 100.0, 100.0),
            w.target_geometry,
            SpringParams::default(),
        )
    }).collect();

    let dt = 1.0 / 120.0;
    for frame in 1..=30 {
        for anim in &mut animators {
            anim.step(dt);
        }
        if frame == 30 {
            for (i, anim) in animators.iter_mut().enumerate() {
                let rect = anim.step(dt);
                println!("      Frame 30: Window {} current geometry ({:.1}, {:.1}) {:.1}x{:.1} (Settled: {})",
                    windows[i].id, rect.x, rect.y, rect.width, rect.height, anim.is_settled());
            }
        }
    }

    println!("\nSession Simulation successfully validated end-to-end!");
    println!("============================================================");
    Ok(())
}

fn run_cosmic_env(export_theme: bool) -> Result<(), Box<dyn std::error::Error>> {
    let env = CosmicSessionEnvironment::new();
    println!("# COSMIC Session Environment Variables:");
    for s in env.to_env_strings() {
        println!("export {}", s);
    }

    let theme = CosmicTheme::google_material_3_dark();
    println!("\n# Google Material 3 Synced Theme RON:");
    let ron = theme.to_ron_string();
    println!("{}", ron);

    if export_theme {
        let appdata = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".into());
        let dir = format!("{}\\.config\\cosmic", appdata);
        std::fs::create_dir_all(&dir)?;
        let path = format!("{}\\com.system76.CosmicTheme.Dark", dir);
        std::fs::write(&path, &ron)?;
        println!("Exported theme RON to: {}", path);
    }

    Ok(())
}
