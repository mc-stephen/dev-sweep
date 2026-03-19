slint::include_modules!();

pub mod modules;
pub mod utils;

// use slint::{Timer, TimerMode};
// use std::time::Duration;

use modules::window_management::WindowManagement;

use crate::modules::dashboard::dashboard::DashboardModule;

fn main() -> Result<(), slint::PlatformError> {
    let window: AppWindow = AppWindow::new()?;
    let ui_handle: slint::Weak<AppWindow> = window.as_weak();

    //==========================
    // Create a repeating timer
    //==========================
    // let timer = Timer::default();
    // timer.start(TimerMode::Repeated, Duration::from_millis(100), {
    //     let window = window.clone_strong();
    //     move || {
    //         // Now this runs periodically on the UI thread
    //         let width = window.get_card_width();
    //         println!("Card width: {}", width);
    //     }
    // });

    //==========================
    //
    //==========================
    WindowManagement {
        ui_handle: ui_handle.clone(),
        window: window.clone_strong(),
    }
    .setup_callbacks();

    //==========================
    //
    //==========================
    let dashboard_module: DashboardModule = DashboardModule::new(window.clone_strong());
    dashboard_module.update_stats();

    window.run()
}
