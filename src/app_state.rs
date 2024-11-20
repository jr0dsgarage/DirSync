use tui::widgets::ListState;
use std::time::{Duration, Instant};
use std::fs;

pub struct AppState {
    pub local_state: ListState,
    pub remote_state: ListState,
    pub local_directories: Vec<String>,
    pub remote_directories: Vec<String>,
    pub current_local_dir: String,
    pub current_remote_dir: String,
    pub output: String,
    pub last_click_time: Instant,
    pub double_click_threshold: Duration,
}

impl AppState {
    pub fn new() -> Self {
        let mut local_state = ListState::default();
        local_state.select(Some(0));
        let mut remote_state = ListState::default();
        remote_state.select(Some(0));
        Self {
            local_state,
            remote_state,
            local_directories: Self::get_mounted_directories(),
            remote_directories: Self::get_mounted_directories(),
            current_local_dir: String::from("C:\\"),
            current_remote_dir: String::from("C:\\"),
            output: String::new(),
            last_click_time: Instant::now(),
            double_click_threshold: Duration::from_millis(500),
        }
    }

    pub fn get_directories(path: &str) -> Vec<String> {
        let mut directories = vec![];
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            if let Some(dir_name) = entry.path().to_str() {
                                directories.push(dir_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        directories
    }

    fn get_mounted_directories() -> Vec<String> {
        let mut directories = vec![];
        let drives = unsafe { winapi::um::fileapi::GetLogicalDrives() };
        for i in 0..26 {
            if drives & (1 << i) != 0 {
                let drive_letter = format!("{}:\\", (b'A' + i) as char);
                directories.push(drive_letter);
            }
        }
        directories
    }
}