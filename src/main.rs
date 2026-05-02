#![cfg_attr(windows, windows_subsystem = "windows")] // hide console on windows

const VERSION: &str = "0.3.1";

use dioxus_native::prelude::*;
use std::any::Any;
use std::sync::OnceLock;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use winit::dpi::LogicalSize;
#[cfg(target_os = "windows")]
use winit::icon::RgbaIcon;
use winit::window::WindowAttributes;
use winit_core::icon::Icon;

mod channel;
mod context;
mod decrypt;
mod encrypt;
#[cfg(target_os = "windows")]
mod icon;
mod pin;
mod startup;
mod theme;
mod ui;
mod utils;
mod wallet;
mod ws;
mod enterpin; 
mod update; 
mod bridge; 

use crate::channel::{WSCommand, Theme};
use crate::context::GlobalContext;
#[cfg(target_os = "windows")]
use crate::icon::load_icon;
use crate::startup::init_startup;
use crate::theme::{DARK_CSS, LIGHT_CSS};
use crate::ws::{run_crypto_websocket, run_exchange_websocket};
use crate::update::UpdatePrompt;
use crate::enterpin::PinScreen;

static UI_COMMANDS_TX: OnceLock<mpsc::Sender<WSCommand>> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppState {
    PinEntry,
    Dashboard,
    UpdatePrompt,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    startup::init_globals();

    println!("Starting main - before init_startup");

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    let handle = runtime.handle().clone();

    init_startup(&handle);

    let (tx_ex, rx_ex) = mpsc::channel::<()>(1);
    let _ = crate::ws::EXCHANGE_SHUTDOWN_TX.set(tx_ex);

    let (commands_tx, commands_rx) = mpsc::channel::<WSCommand>(100);
    let (ledger_reg_tx, ledger_reg_rx) = mpsc::channel(10);
    let (outgoing_tx, outgoing_rx) = mpsc::channel(100);
    let (crypto_shutdown_tx, crypto_shutdown_rx) = mpsc::channel::<()>(1);

    let _ = UI_COMMANDS_TX.set(commands_tx.clone());
    let tx_for_wallet = commands_tx.clone();

    let _ = crate::ws::CRYPTO_COMMANDS_TX.set(commands_tx);
    let _ = crate::ws::LEDGER_REGISTRY_TX.set(ledger_reg_tx);
    let _ = crate::ws::CRYPTO_OUTGOING_TX.set(outgoing_tx);
    let _ = crate::ws::CRYPTO_SHUTDOWN_TX.set(crypto_shutdown_tx);

    let mut join_handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    let exchange_handle = handle.spawn(async move {
        if let Err(_e) = run_exchange_websocket(rx_ex).await {
            println!("Exchange websocket error: {:?}", _e);
        }
    });
    join_handles.push(exchange_handle);

    let crypto_handle = handle.spawn(async move {
        if let Err(_e) = run_crypto_websocket(
            commands_rx,
            ledger_reg_rx,
            outgoing_rx,
            crypto_shutdown_rx,
        )
        .await
        {
            println!("Crypto websocket error: {:?}", _e);
        }
    });
    join_handles.push(crypto_handle);

    let wallet_handle = handle.spawn_blocking(move || {
        wallet::load_wallets(tx_for_wallet);
    });
    join_handles.push(wallet_handle);

    #[cfg(target_os = "windows")]
    let window_icon = {
        let icon_data = load_icon()?;
        let rgba_icon = RgbaIcon::new(icon_data.rgba, icon_data.width, icon_data.height)?;
        Some(Icon::from(rgba_icon))
    };

    #[cfg(target_os = "linux")]
    let window_icon: Option<Icon> = None;

    #[cfg(target_os = "windows")]
    let default_size = LogicalSize::new(1120.0, 640.0);
    #[cfg(target_os = "linux")]
    let default_size = LogicalSize::new(1400.0, 800.0);

    #[cfg(target_os = "linux")]
    let mut window_attr = WindowAttributes::default()
        .with_title("Dannesk")
        .with_surface_size(default_size)
        .with_resizable(true)
        .with_window_icon(window_icon);

    #[cfg(target_os = "windows")]
    let window_attr = WindowAttributes::default()
        .with_title("Dannesk")
        .with_surface_size(default_size)
        .with_resizable(true)
        .with_window_icon(window_icon);

    #[cfg(target_os = "linux")]
    {
        use winit_core::window::PlatformWindowAttributes;
        use winit_wayland::WindowAttributesWayland;
        use winit_x11::WindowAttributesX11;

        let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
        let platform_attr: Box<dyn PlatformWindowAttributes> = if session_type == "wayland" {
            Box::new(WindowAttributesWayland::default().with_name("dannesk", "dannesk"))
        } else {
            Box::new(WindowAttributesX11::default().with_name("Dannesk", "dannesk"))
        };
        window_attr = window_attr.with_platform_attributes(platform_attr);
    }

    println!("Launching Dioxus app");
    dioxus_native::launch_cfg(App, vec![], vec![Box::new(window_attr) as Box<dyn Any>]);
    println!("Dioxus app exited");

    handle.block_on(async {
        println!("Sending websocket shutdown signals.");
        if let Some(tx) = crate::ws::EXCHANGE_SHUTDOWN_TX.get() {
            let _ = tx.send(()).await;
        }
        if let Some(tx) = crate::ws::CRYPTO_SHUTDOWN_TX.get() {
            let _ = tx.send(()).await;
        }
        for jh in join_handles {
            let _ = jh.await;
        }
        println!("All tasks completed.");
    });

    Ok(())
}

#[component]
fn App() -> Element {
    let tx = UI_COMMANDS_TX
        .get()
        .expect("UI_COMMANDS_TX not set")
        .clone();
    context::setup_contexts(tx);

    let global = use_context::<GlobalContext>();
    let (theme, _hide_balance) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);

    let mut unlocked = use_signal(|| false);
    let remote_version = global.version.read();

    let current_view = match remote_version.as_ref() {
        Some(v) if v != VERSION => AppState::UpdatePrompt,
        _ => {
            if *unlocked.read() {
                AppState::Dashboard
            } else {
                AppState::PinEntry
            }
        }
    };

    let theme_css = if is_dark { DARK_CSS } else { LIGHT_CSS };

    rsx! {
        style { "body {{ margin: 0; padding: 0; }} {theme_css}" }
        div {
            class: "theme-root",
            class: if is_dark { "dark" },
            style: "display: flex; flex-direction: column; height: 100vh; width: 100%; overflow: hidden;",
            div {
                style: "display: flex; flex-direction: column; flex: 1; width: 100%; margin: auto;",
                match current_view {
                    AppState::UpdatePrompt => rsx! { UpdatePrompt {} },
                    AppState::PinEntry => rsx! {
                        PinScreen { on_unlock: move |_| unlocked.set(true) }
                    },
                    AppState::Dashboard => rsx! {
                        ui::dashboard::render_dashboard {}
                    }
                }
            }
        }
    }
}