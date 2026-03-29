// ADD THIS: Import from the crate root where they were generated
use crate::{AppWindow, AppWindowData, WindowControl};

use i_slint_backend_winit::WinitWindowAccessor;
use slint::ComponentHandle;

pub struct WindowManagement {
    pub window: AppWindow,
    pub ui_handle: slint::Weak<AppWindow>,
}

impl WindowManagement {
    pub fn setup_callbacks(&self) {
        // --- Drag Window ---
        self.window.global::<AppWindowData>().on_start_system_move({
            let ui_handle: slint::Weak<AppWindow> = self.ui_handle.clone();
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
        self.window.global::<WindowControl>().on_close_requested({
            let ui_handle: slint::Weak<AppWindow> = self.ui_handle.clone();
            move || {
                if let Some(win) = ui_handle.upgrade() {
                    let _ = win.hide();
                }
            }
        });

        // --- Minimize ---
        self.window
            .global::<WindowControl>()
            .on_minimize_requested({
                let ui_handle: slint::Weak<AppWindow> = self.ui_handle.clone();
                move || {
                    if let Some(win) = ui_handle.upgrade() {
                        win.window().set_minimized(true);
                    }
                }
            });

        // --- Maximize ---
        self.window
            .global::<WindowControl>()
            .on_maximize_requested({
                let ui_handle: slint::Weak<AppWindow> = self.ui_handle.clone();
                move || {
                    if let Some(win) = ui_handle.upgrade() {
                        let is_max: bool = win.window().is_maximized();
                        win.window().set_maximized(!is_max);
                    }
                }
            });
    }
}
