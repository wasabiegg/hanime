use std::path::Path;
use tokio::fs;
use tokio::io;

pub async fn clean_folder(tmp_path: &Path) -> io::Result<()> {
    fs::remove_dir_all(tmp_path).await
}
