slint::include_modules!();

pub mod modules;
pub mod utils;

use serde_json::Value;
use std::{
    fs::{self, ReadDir, read_dir, DirEntry},
    rc::Rc,
};

use base64::{Engine as _, engine::general_purpose};

use modules::window_management::WindowManagement;
use slint::{Color, Image, ModelRc, Rgba8Pixel, SharedPixelBuffer, ToSharedString, VecModel};

use crate::modules::dashboard::dashboard::DashboardModule;

// use std::io::Error;

// use slint::{Timer, TimerMode};
// use std::time::Duration;

fn main() -> Result<(), slint::PlatformError> {
    let window: AppWindow = AppWindow::new()?;
    let ui_handle: slint::Weak<AppWindow> = window.as_weak();

    // Cleaner Schema Config Dummy Path
    let prod_path: String = "~/.config/devsweep/cleaners/".to_owned();
    let dev_path: String = "/Users/apple/projects/rust/dev_sweep_lint/assets/configs".to_owned();

    let cleaner_path: String = dev_path.to_owned();
    let configs: ReadDir = read_dir(cleaner_path).unwrap();

    let mut cleaners: Vec<Cleaner> = vec![];

    for config_result in configs {

        if let Err(err) = &config_result {
            dbg!(err);
            continue; // 👈 skip this iteration
        }

        let config: DirEntry = config_result.unwrap(); // safe now
        dbg!(config.file_name());

        let config_content: String = fs::read_to_string(config.path()).unwrap();
        let config_json: Value = serde_json::from_str(&config_content).unwrap();

        // prefix 0xAA or 0xFF - This convert Hex string to u32
        let color: String = config_json["color"].to_string();
        let clean_color_hex: &str = color.trim_start_matches('#');
        let rgb: u32 = u32::from_str_radix(clean_color_hex, 16).unwrap_or(0);
        let alpha: u32 = 0xFF << 24; // Add the Alpha channel (0xFF) at the very front
        let complete_u32: u32 = alpha | rgb;

        // This convert string to image
        let b64: String = config_json["icon"].to_string();
        let bytes: Vec<u8> = general_purpose::STANDARD.decode(&b64).expect(""); // Decode string to bytes
        let img = image::load_from_memory(&bytes).expect("Failed to load image"); // Load bytes into Slint
        let rgba = img.to_rgba8(); // (using the 'image' crate to parse format)
        let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(rgba.width(), rgba.height());
        buffer.make_mut_bytes().copy_from_slice(rgba.as_raw());

        cleaners.push(Cleaner {
            detect: [].into(),
            clean_targets: [].into(),
            icon: Image::from_rgba8(buffer),
            color: Color::from_argb_encoded(complete_u32),
            id: config_json["id"].to_shared_string().into(),
            name: config_json["name"].to_shared_string().into(),
            description: config_json["description"].to_shared_string().into(),
            clean_command: config_json["clean_command"].to_shared_string().into(),
            restore_command: config_json["restore_command"].to_shared_string().into(),
            estimated_savings: config_json["estimated_savings"].to_shared_string().into(),
        });
    }

    let model: slint::ModelRc<Cleaner> = ModelRc::from(Rc::new(VecModel::from(cleaners)));
    window.global::<CleanerGlobal>().set_cleaners(model);

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
