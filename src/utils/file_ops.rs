use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileOps;

impl FileOps {
    pub fn find_files_by_extension(
        root_path: &Path,
        extension: &str,
        recursive: bool,
    ) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if recursive {
            for entry in WalkDir::new(root_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
                    files.push(path.to_path_buf());
                }
            }
        } else {
            for entry in std::fs::read_dir(root_path)? {
                let path = entry?.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
                    files.push(path);
                }
            }
        }

        files.sort();
        Ok(files)
    }

    pub fn create_output_directory(output_path: &Path) -> Result<()> {
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }
        Ok(())
    }

    pub fn ensure_extension(path: &Path, extension: &str) -> PathBuf {
        if path.extension().map_or(true, |ext| ext != extension) {
            let mut new_path = path.to_path_buf();
            new_path.set_extension(extension);
            new_path
        } else {
            path.to_path_buf()
        }
    }

    pub fn get_relative_path(full_path: &Path, base_path: &Path) -> Result<PathBuf> {
        full_path.strip_prefix(base_path)
            .map(|p| p.to_path_buf())
            .with_context(|| format!("Failed to get relative path for {:?}", full_path))
    }

    pub fn copy_file(src: &Path, dest: &Path) -> Result<()> {
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(src, dest)
            .with_context(|| format!("Failed to copy file from {:?} to {:?}", src, dest))?;
        Ok(())
    }

    pub fn get_file_name_without_extension(path: &Path) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn is_image_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg")
        } else {
            false
        }
    }

    pub fn find_image_files(root_path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if recursive {
            for entry in WalkDir::new(root_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && Self::is_image_file(&path) {
                    files.push(path.to_path_buf());
                }
            }
        } else {
            for entry in std::fs::read_dir(root_path)? {
                let path = entry?.path();
                if path.is_file() && Self::is_image_file(&path) {
                    files.push(path);
                }
            }
        }

        files.sort();
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_find_files_by_extension() {
        let temp_dir = tempdir().unwrap();
        let dir_path = temp_dir.path();

        File::create(dir_path.join("test1.md")).unwrap();
        File::create(dir_path.join("test2.md")).unwrap();
        File::create(dir_path.join("ignore.txt")).unwrap();

        let files = FileOps::find_files_by_extension(dir_path, "md", false).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.file_name().unwrap() == "test1.md"));
        assert!(files.iter().any(|f| f.file_name().unwrap() == "test2.md"));
    }

    #[test]
    fn test_ensure_extension() {
        let path = Path::new("output");
        let result = FileOps::ensure_extension(path, "json");
        assert_eq!(result.to_string_lossy(), "output.json");

        let path = Path::new("output.json");
        let result = FileOps::ensure_extension(path, "json");
        assert_eq!(result.to_string_lossy(), "output.json");
    }

    #[test]
    fn test_is_image_file() {
        assert!(FileOps::is_image_file(Path::new("test.jpg")));
        assert!(FileOps::is_image_file(Path::new("test.JPG")));
        assert!(FileOps::is_image_file(Path::new("test.png")));
        assert!(!FileOps::is_image_file(Path::new("test.txt")));
    }

    #[test]
    fn test_get_file_name_without_extension() {
        assert_eq!(FileOps::get_file_name_without_extension(Path::new("test.md")), "test");
        assert_eq!(FileOps::get_file_name_without_extension(Path::new("path/to/file.ext")), "file");
    }
}