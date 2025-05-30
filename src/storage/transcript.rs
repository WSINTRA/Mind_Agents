// src/storage/transcript.rs
use chrono::Local;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct TranscriptManager {
    output_dir: PathBuf,
}

impl TranscriptManager {
    pub fn new() -> io::Result<Self> {
        let output_dir = PathBuf::from("transcripts");
        fs::create_dir_all(&output_dir)?;

        Ok(TranscriptManager { output_dir })
    }

    pub fn save_transcript(&self, transcript: &str) -> io::Result<PathBuf> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("conversation_{}.txt", timestamp);
        let file_path = self.output_dir.join(filename);

        let mut file = File::create(&file_path)?;

        // Write timestamp and transcript
        writeln!(
            file,
            "Transcript Date: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(file, "{}", transcript)?;

        println!("Transcript saved to {}", file_path.display());
        Ok(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    fn cleanup_test_file(path: &PathBuf) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_transcript_manager_creation() {
        let manager = TranscriptManager::new().unwrap();
        assert!(manager.output_dir.exists());
        assert!(manager.output_dir.is_dir());
    }

    #[test]
    fn test_save_transcript() {
        let manager = TranscriptManager::new().unwrap();
        let test_content = "Test conversation content";

        let file_path = manager.save_transcript(test_content).unwrap();

        // Verify file exists and contains content
        let mut file = File::open(&file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        assert!(content.contains(test_content));
        assert!(content.contains("Transcript Date:"));

        cleanup_test_file(&file_path);
    }

    #[test]
    fn test_file_naming() {
        let manager = TranscriptManager::new().unwrap();
        let test_content = "Test content";

        let file_path = manager.save_transcript(test_content).unwrap();

        // Verify filename format
        assert!(file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("conversation_"));
        assert!(file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with(".txt"));

        cleanup_test_file(&file_path);
    }
}
