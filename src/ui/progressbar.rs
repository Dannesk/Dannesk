use crate::channel::{CHANNEL, ProgressState};
use crate::context::GlobalContext;
use dioxus_native::prelude::*;
use std::time::Duration;

#[component]
pub fn ProgressBar(operation_name: String) -> Element {
    let global = use_context::<GlobalContext>();
    let state_signal = global.progress;

    // 1. ANIMATION & LERP STATE
    let mut render_progress = use_signal(|| 0.0f32);
    let mut frame_count = use_signal(|| 0u64);
    let mut flicker_alpha = use_signal(|| 1.0f32);

    // 2. THE UNIFIED FRAME DRIVE (60FPS)
    use_future(move || async move {
        let mut interval = tokio::time::interval(Duration::from_millis(16));
        let mut finished_at: Option<std::time::Instant> = None;

        loop {
            interval.tick().await;

            let current_state = state_signal.read().clone();
            let target = current_state.as_ref().map(|s| s.progress).unwrap_or(0.0);

            render_progress.with_mut(|v| {
                let diff = target - *v;
                if diff.abs() > 0.001 { *v += diff * 0.1; } 
                else { *v = target; }
            });

            frame_count.with_mut(|f| *f += 1);
            let pulse = ((frame_count() as f32 * 0.2).sin() * 0.1) + 0.9;
            flicker_alpha.set(pulse);

            if let Some(ref s) = current_state {
                let is_done = s.progress >= 1.0 
                    || s.message.to_lowercase().contains("error") 
                    || s.message.to_lowercase().contains("failed");

                if is_done {
                    if finished_at.is_none() { finished_at = Some(std::time::Instant::now()); }
                    if finished_at.unwrap().elapsed() >= Duration::from_millis(300) {
                        let _ = CHANNEL.progress_tx.send(None);
                        break; 
                    }
                }
            }
        }
    });

    // 3. TIMEOUT HANDLER
    use_future(move || async move {
        tokio::time::sleep(Duration::from_secs(15)).await;
        let s = state_signal.read();
        if let Some(ref state) = *s {
            if state.progress < 1.0 {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "SYSTEM_TIMEOUT // PLEASE_RETRY".to_string(),
                }));
            }
        }
    });

    // --- PRE-COMPUTE UI DATA ---
    let state_lock = state_signal.read();
    let Some(ref state) = *state_lock else { return rsx! {} };

    // Hex replacements from DARK_CSS
    let status_color = if state.message.to_lowercase().contains("error") { "#ef4444" } else { "#ffffff" };
    let chars = ["|", "/", "-", "\\"];
    let spinner_char = chars[(frame_count() / 8 % 4) as usize];
    let display_percent = (render_progress() * 100.0) as i32;
    let scanline_top = (frame_count() % 100) as f32;

    let mut segments = Vec::with_capacity(24);
    for i in 0..24 {
        let color = if (i as f32 / 24.0) < render_progress() { status_color } else { "transparent" };
        segments.push(color);
    }

    rsx! {
        div { 
            class: "terminal-overlay",
            style: "opacity: {flicker_alpha}; position: fixed; top: 0; left: 0; width: 100%; height: 100%; display: flex; justify-content: center; align-items: center; background: rgba(0,0,0,0.85);",
            
            div { 
                class: "terminal-box",
                style: "width: 400px; padding: 24px; background: #0a0a0a; border: 1px solid #262626; position: relative; font-family: monospace;",
                
                div { 
                    style: "display: flex; justify-content: space-between; color: {status_color}; margin-bottom: 8px;",
                    span { "PROCESS // {operation_name.to_uppercase()}" }
                    span { "[{spinner_char}] {display_percent}%" }
                }

                div { 
                    style: "display: flex; height: 20px; width: 100%; background: #000; border: 1px solid #222; padding: 2px;",
                    for color in segments {
                        div { style: "flex: 1; margin: 1px; background-color: {color};" }
                    }
                }

                div { 
                    style: "margin-top: 12px; font-size: 11px; color: {status_color};",
                    "> {state.message.to_uppercase()}"
                }
                
                div {
                    style: "position: absolute; left: 0; width: 100%; height: 2px; background: {status_color}; opacity: 0.1; pointer-events: none; top: {scanline_top}%;"
                }
            }
        }
    }
}