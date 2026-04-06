use crate::{
    AppWindow, SettingsData, Theme, ThemeType,
    utils::{database::database::DB, services::variables::Variables},
};
use rusqlite::Connection;
use slint::{Color, ComponentHandle, Model};
use std::{cell::RefCell, rc::Rc, sync::MutexGuard};

pub struct Settings {
    window: AppWindow,
}

impl Settings {
    //=========================
    //
    //=========================
    pub fn init(self) {
        let this: Rc<RefCell<Settings>> = Rc::new(RefCell::new(self));

        let window: AppWindow = this.borrow().window.clone_strong();
        let settings_page: SettingsData<'_> = window.global::<SettingsData>();

        //********************* Auto populate settings current value **********
        settings_page.set_app_version("1.0.0".into());
        for val in SettingsType::all() {
            this.borrow_mut().update_ui(val);
        }

        //*********************
        let update_db = |name: String, value: String| {
            let sql_err: &str = "Failed to update Settings table";
            let sql: &str = "UPDATE settings SET value = ?1 WHERE name = ?2";
            let db: MutexGuard<'_, Connection> = DB.get().unwrap().lock().unwrap();
            db.execute(sql, (&value, &name)).expect(sql_err);
        };

        //********************* Theme logic scope ********************* 36 x 18 x 29
        settings_page.on_update_theme({
            let this_clone: Rc<RefCell<Settings>> = this.clone();
            move |on_theme: ThemeType| {
                // update_db("".into(), "".into());
                // this_clone.borrow_mut().update_ui(&SettingsType::ThemeIndex);
            }
        });

        //*********************  *********************
        settings_page.on_update_accent_colors(move |color: Color| {
            //
        });

        //*********************  *********************
        settings_page.on_update_compact_mode(move |value| {
            //
        });

        //******************************************
        settings_page.on_update_dry_run(move |value| {
            //
        });

        //*********************  *********************
        settings_page.on_update_skip_git_repo(move |value| {
            //
        });

        //*********************  *********************
        settings_page.on_update_exclude_path(move |value| {
            //
        });

        //*********************  *********************
        settings_page.on_update_cleaner_config_path(move || {
            //
        });

        //*********************  *********************
        settings_page.on_update_auto_reload_configs(move |value| {
            //
        });

        //********************* Open Link *********************
        settings_page.on_click_project_links(move |link| {
            open::that(link.as_str()).expect("Failed to open link in system browser");
        });
    }

    //=========================
    //
    //=========================
    fn update_ui(&mut self, which: &SettingsType) {
        let theme: Theme<'_> = self.window.global::<Theme>();
        let settings_page: SettingsData<'_> = self.window.global::<SettingsData>();

        //=========================
        let bool_type = |val: &str| -> bool {
            match val.to_lowercase().as_str() {
                "1" => true,
                "0" => false,
                "true" => true,
                "false" => false,
                _ => false,
            }
        };

        //=========================
        let get_db_value = |name: String| {
            let sql_err: &str = "Failed to get Settings DB value from row";
            let sql: &str = "SELECT value FROM settings WHERE name = ?1 LIMIT 1";
            let db: MutexGuard<'_, Connection> = DB.get().unwrap().lock().unwrap();
            let table: Result<String, rusqlite::Error> = db.query_one(sql, [name], |row| {
                let v: String = row.get(0).expect(sql_err);
                Ok(v)
            });

            table.expect("Failed to get Settings DB value").to_owned()
        };

        match which {
            SettingsType::DryRun => {
                let value: String = get_db_value("dry_run".into());
                settings_page.set_dry_run(bool_type(value.as_str()));
            }
            SettingsType::AccentIndex => {
                let value: String = get_db_value("accent_index".into());
                let accents: Vec<Color> = theme.get_accents().iter().collect();
                let index: usize = value.parse::<usize>().expect("Invalid Accent index found");
                theme.set_accent(accents[index]);
            }
            SettingsType::ThemeIndex => {
                let value: String = get_db_value("theme_index".into());
                let themes: Vec<ThemeType> = theme.get_themes().iter().collect();
                let index: usize = value.parse::<usize>().expect("Invalid Theme index found");
                theme.set_theme(themes[index]);
            }
            SettingsType::CompactMode => {
                let value: String = get_db_value("compact_mode".into());
                settings_page.set_compact_mode(bool_type(value.as_str()));
            }
            SettingsType::SkipGitRepo => {
                let value: String = get_db_value("skip_git_repo".into());
                settings_page.set_skip_git_repo(bool_type(value.as_str()));
            }
            SettingsType::ExcludePath => {
                let value: String = get_db_value("exclude_path".into());
                settings_page.set_exclude_path(value.into());
            }
            SettingsType::AutoReloadConfigs => {
                let value: String = get_db_value("auto_reload_configs".into());
                settings_page.set_auto_reload_configs(bool_type(value.as_str()));
            }
            SettingsType::CleanerConfigPath => {
                let value: String = get_db_value("cleaner_config_path".into());
                settings_page.set_cleaner_config_path(value.into());
            }
        }
    }

    //=========================
    //
    //=========================
    pub fn new(window: AppWindow) -> Self {
        let db: MutexGuard<'_, Connection> = DB.get().unwrap().lock().unwrap();

        let err_msg: &str = "Failed to create `Settings` table";
        let table_cmd: &str = "CREATE TABLE IF NOT EXISTS settings (
            name TEXT PRIMARY KEY,
            value TEXT
        ) WITHOUT ROWID;";
        db.execute(table_cmd, []).expect(&err_msg);

        let vars: Variables = Variables::default();
        let settings: [(&str, &str); 8] = [
            ("dry_run", "false"),                             // true, false
            ("theme_index", "1"),                             // 1, 0
            ("accent_index", "0"),                            // 1, 0
            ("compact_mode", "false"),                        // true, false
            ("skip_git_repo", "false"),                       // true, false
            ("auto_reload_configs", "false"),                 // true, false
            ("cleaner_config_path", &vars.cleaner_path),      // String*
            ("exclude_path", &vars.excluded_path.join("\n")), // String*
        ];

        let err_msg: &str = "Failed to create `Settings` rows";
        let row_cmd: &str = "INSERT OR IGNORE INTO settings (name, value) VALUES (?1, ?2)";
        for (name, value) in settings {
            db.execute(row_cmd, (name, value)).expect(&err_msg);
        }

        Self { window: window }
    }
}

enum SettingsType {
    DryRun,
    ThemeIndex,
    AccentIndex,
    CompactMode,
    SkipGitRepo,
    ExcludePath,
    AutoReloadConfigs,
    CleanerConfigPath,
}

impl SettingsType {
    fn all() -> &'static [SettingsType] {
        &[
            SettingsType::DryRun,
            SettingsType::ThemeIndex,
            SettingsType::AccentIndex,
            SettingsType::CompactMode,
            SettingsType::SkipGitRepo,
            SettingsType::ExcludePath,
            SettingsType::AutoReloadConfigs,
            SettingsType::CleanerConfigPath,
        ]
    }
}
