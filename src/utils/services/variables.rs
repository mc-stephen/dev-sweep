use std::path::PathBuf;

pub struct Variables {
    pub build_path: String,
    pub cleaner_path: String,
    pub excluded_path: Vec<String>,
}

impl Default for Variables {
    fn default() -> Self {
        let build_path: PathBuf = match cfg!(debug_assertions) {
            true => PathBuf::from("./assets/devsweep"),
            false => dirs::config_dir().unwrap().join(".config/devsweep"),
        };

        let cleaner_path: PathBuf = build_path.join("configs");

        Self {
            build_path: build_path.to_string_lossy().into(),
            cleaner_path: cleaner_path.to_string_lossy().into(),
            excluded_path: vec!["/usr".into(), "/System".into(), "/Library".into()],
        }
    }
}
