use std::path::PathBuf;

pub fn db_foler_path(data_dir: &str, db_name: &str) -> PathBuf {
    PathBuf::from(data_dir).join(db_name)
}
