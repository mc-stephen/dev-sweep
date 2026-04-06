use crate::utils::services::variables::Variables;
use rusqlite::Connection;
use std::path::Path;

use std::sync::{Mutex, OnceLock};

// works best for single user app
pub static DB: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn init_db() {
    let db_name: &str = "/database/app.db";
    let biddings: String = Variables::default().build_path + db_name;
    let db_path: &Path = Path::new(&biddings);
    let connection: Connection = Connection::open(db_path).expect("Expected a valid DB path");

    DB.set(Mutex::new(connection)).unwrap();
}

//  DB.get().unwrap().lock().unwrap();