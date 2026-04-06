use crate::{Cleaner, utils::services::variables::Variables};
use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use slint::{Color, Image, Rgba8Pixel, SharedPixelBuffer};
use std::fs::{self, DirEntry, ReadDir, read_dir};

//==============================
//
//==============================
pub fn get_cleaner_configs() -> Vec<Cleaner> {
    let mut cleaners: Vec<Cleaner> = Vec::new();
    let configs: ReadDir = read_dir(Variables::default().cleaner_path).unwrap();

    for config_result in configs {
        if let Err(err) = &config_result {
            dbg!(err);
            continue; // 👈 skip this iteration
        }

        let config: DirEntry = config_result.unwrap();
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
        let b64: &str = config_json["icon"]
            .as_str()
            .expect("Expecting a string like value");
        let bytes: Vec<u8> = general_purpose::STANDARD
            .decode(&b64)
            .expect("Expect. value base64 value");
        let img: image::DynamicImage =
            image::load_from_memory(&bytes).expect("Failed to load image");
        let rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8(); // (using the 'image' crate to parse format)
        let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(rgba.width(), rgba.height());
        buffer.make_mut_bytes().copy_from_slice(rgba.as_raw());

        cleaners.push(Cleaner {
            detect: [].into(),
            clean_targets: [].into(),
            icon: Image::from_rgba8(buffer),
            color: Color::from_argb_encoded(complete_u32),
            id: config_json["id"].as_str().expect("msg").into(),
            name: config_json["name"].as_str().expect("msg").into(),
            description: config_json["description"].as_str().expect("msg").into(),
            clean_command: config_json["clean_command"].as_str().expect("msg").into(),
            restore_command: config_json["restore_command"].as_str().expect("msg").into(),
            estimated_savings: config_json["estimated_savings"]
                .as_str()
                .expect("msg")
                .into(),
        });
    }

    cleaners.to_owned()
}
