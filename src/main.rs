slint::include_modules!();

pub mod modules;
pub mod utils;

use crate::modules::activity::Activity;
use crate::modules::cleaner::Schema;
use crate::modules::dashboard::Dashboard;
use crate::modules::settings::Settings;
use crate::utils::database::database;
use modules::others::configs;
use modules::others::window_management;
// use std::io::Error;

// use slint::{Timer, TimerMode};
// use std::time::Duration;

fn main() -> Result<(), slint::PlatformError> {
    let window: AppWindow = AppWindow::new()?;
    let ui_handle: slint::Weak<AppWindow> = window.as_weak();

    //===========================
    //
    //===========================
    database::init_db(); // 0. Init Database
    let cleaners: Vec<Cleaner> = configs::get_cleaner_configs(); // 1. Get all Cleaner Schema
    window_management::handle_window(window.clone_strong(), ui_handle.clone()); // 2. Handing Window Management

    //===========================
    //
    //===========================
    Activity::new(window.clone_strong()).init();
    Settings::new(window.clone_strong()).init();
    Dashboard::new(window.clone_strong()).init();
    Schema::new(window.clone_strong(), cleaners).init();

    //===========================
    //
    //===========================

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

    window.run()
}
