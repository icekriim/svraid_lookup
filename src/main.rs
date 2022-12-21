#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[cfg(not(target_arch = "wasm32"))]
use eframe::Renderer;

use sv_raid_lookup::app::SVRaidLookup;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        vsync: false,
        renderer: Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "SV Raid Lookup",
        native_options,
        Box::new(|cc| Box::new(SVRaidLookup::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "sv_raid_lookup", // hardcode it
            web_options,
            Box::new(|cc| Box::new(SVRaidLookup::new(cc))),
        )
        .await
        .expect("Failed to start eframe");
    });
}
