use crate::{AppWindow, AppWindowData, WindowControl};
use i_slint_backend_winit::WinitWindowAccessor;
use slint::{ComponentHandle, Weak};

pub fn handle_window(window: AppWindow, ui_handle: Weak<AppWindow>) {
    // --- Drag Window ---
    window.global::<AppWindowData>().on_start_system_move({
        let ui_handle: slint::Weak<AppWindow> = ui_handle.clone();
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.window().with_winit_window(|winit_window| {
                    let _ = winit_window.drag_window();
                });
            }
        }
    });

    // --- Close (Using Global) ---
    // If Rust still says "not found", use: main_window.global::<WindowControl>()
    // and make sure 'export global WindowControl' is in your .slint
    window.global::<WindowControl>().on_close_requested({
        let ui_handle: slint::Weak<AppWindow> = ui_handle.clone();
        move || {
            if let Some(win) = ui_handle.upgrade() {
                let _ = win.hide();
            }
        }
    });

    // --- Minimize ---
    window.global::<WindowControl>().on_minimize_requested({
        let ui_handle: slint::Weak<AppWindow> = ui_handle.clone();
        move || {
            if let Some(win) = ui_handle.upgrade() {
                win.window().set_minimized(true);
            }
        }
    });

    // --- Maximize ---
    window.global::<WindowControl>().on_maximize_requested({
        let ui_handle: slint::Weak<AppWindow> = ui_handle.clone();
        move || {
            if let Some(win) = ui_handle.upgrade() {
                let is_max: bool = win.window().is_maximized();
                win.window().set_maximized(!is_max);
            }
        }
    });
}
