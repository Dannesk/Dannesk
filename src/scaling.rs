use std::sync::OnceLock;

pub static SCALE_FACTOR: OnceLock<f64> = OnceLock::new();

#[cfg(target_os = "windows")]
pub fn get_system_scale() -> f64 {
    use windows::Win32::UI::HiDpi::GetDpiForSystem;
    unsafe { GetDpiForSystem() as f64 / 96.0 }
}

#[cfg(target_os = "linux")]
pub fn get_system_scale() -> f64 {
    std::env::var("GDK_SCALE")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or_else(|| {
            std::env::var("WINIT_X11_SCALE_FACTOR")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(1.0)
        })
}

#[cfg(target_os = "macos")]
pub fn get_system_scale() -> f64 {
    2.0 
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn get_system_scale() -> f64 {
    1.0
}