use crate::{AppWindow, DashboardAdapter, StatData};
use slint::{ComponentHandle, Model}; // Generated types
use std::rc::Rc;

// //===============================
// //
// //===============================
// #[derive(Clone, Debug)]
// pub struct Stat {
//     icon: String,
//     value: String,
//     label: String,
//     trend_icon: String,
//     trend_label: String,
// }

// impl Default for Stat {
//     fn default() -> Self {
//         Self {
//             icon: "".to_string(),
//             value: "".to_string(),
//             label: "".to_string(),
//             trend_icon: "".to_string(),
//             trend_label: "".to_string(),
//         }
//     }
// }

//===============================
// If Stat already implements Default, you can let Rust derive everything automatically:
// By adding #[derive(Default)] to StatsDetails
//===============================
// #[derive(Clone, Debug, Default)]
// pub struct StatsDetails {
//     project_found: StatData,
//     total_cleaned: StatData,
//     active_cleaners: StatData,
//     space_recoverable: StatData,
// }

//===============================
//
//===============================
pub struct Dashboard {
    window: AppWindow,
    // quick_actions: [(&'static str, &'static str); 4],
}

impl Dashboard {
    pub fn new(window: AppWindow) -> Self {
        Self {
            window,
            // quick_actions: [
            //     ("Start New Scan", "Scan your filesystem for dev projects"),
            //     ("Quick clean", "Clean all detected projects at once"),
            //     ("Add New Scan Cleaner", "Import a custom cleaner config"),
            //     ("Documentation", "Learn how to create custom cleaners"),
            // ],
        }
    }

    //===============================
    //
    //===============================
    pub fn init(&self) {
        self.update_stats();
    }

    //===============================
    //
    //===============================
    pub fn update_stats(&self) {
        // 1. Get your raw data from your logic
        let raw_stats: Vec<StatData> = self.get_stats();

        // 2. Convert Rust types to Slint types
        // let slint_data: Vec<StatData> = raw_stats
        //     .into_iter()
        //     .map(|s| {
        //         StatData {
        //             label: s.label.into(),
        //             value: s.value.into(),
        //             trend_label: s.trend_label.into(),
        //             // For icons, you can use slint::Image::load_from_path
        //             icon: slint::Image::default(),
        //             trend_icon: slint::Image::default(),
        //         }
        //     })
        //     .collect();

        // 3. Push to the UI
        let shared_model: slint::VecModel<StatData> = slint::VecModel::from(raw_stats);
        let model_rc: slint::ModelRc<StatData> = slint::ModelRc::from(Rc::new(shared_model));
        let dashboard_adapter: DashboardAdapter<'_> = self.window.global::<DashboardAdapter>();
        dashboard_adapter.set_stats(model_rc.into());
    }

    //===============================
    //
    //===============================
    fn get_stats(&self) -> Vec<StatData> {
        // 1. Get the current list from the UI
        let mut current_stats: Vec<StatData> = self
            .window
            .global::<DashboardAdapter>()
            .get_stats()
            .iter()
            .collect();

        // 2. Modify only the first item (Projects Found)
        if let Some(first_stat) = current_stats.get_mut(0) {
            first_stat.value = "15".into();
        }

        current_stats

        // vec![
        //     StatData {
        //         label: "Projects Found".into(),
        //         value: "12".into(),
        //         ..Default::default()
        //     },
        //     StatData {
        //         label: "Space Recoverable".into(),
        //         value: "1.2 GB".into(),
        //         ..Default::default()
        //     },
        //     // ...
        // ]
    }

    // fn print_stats(&self, stats: &StatsDetails) {
    //     for (getter, label) in &self.stat_data {
    //         println!("{}: {}", label, getter(stats));
    //     }
    // }

    // fn print_actions(&self) {
    //     for (title, desc) in &self.quick_actions {
    //         println!("{}: {}", title, desc);
    //     }
    // }

    // fn main() {
    //     let globals = GlobalVariable::new();
    //     let stats = StatsDetails {
    //         project_found: "5".to_string(),
    //         total_cleaned: "10MB".to_string(),
    //         active_cleaners: "2".to_string(),
    //         space_recoverable: "50MB".to_string(),
    //     };

    //     globals.print_stats(&stats);
    //     globals.print_actions();
    // }
}
