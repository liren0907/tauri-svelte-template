pub struct DialogHandler;

impl DialogHandler {
    pub fn select_file() -> Result<String, String> {
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp", "bmp", "svg", "ico", "tiff"])
            .add_filter("Videos", &["mp4", "webm", "mov", "avi", "mkv"])
            .pick_file()
        {
            Ok(file.to_string_lossy().to_string())
        } else {
            Err("No file selected".into())
        }
    }
}
