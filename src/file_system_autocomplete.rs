use inquire::{autocompletion::Replacement, Autocomplete};

#[derive(Clone)]
pub struct FileSystemAutocomplete {
    directories: bool,
    files: bool,
}

impl FileSystemAutocomplete {
    pub fn files() -> Self {
        Self {
            directories: false,
            files: true,
        }
    }

    pub fn directories() -> Self {
        Self {
            directories: true,
            files: false,
        }
    }

    pub fn both() -> Self {
        Self {
            directories: true,
            files: true,
        }
    }
}

impl Autocomplete for FileSystemAutocomplete {
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, inquire::CustomUserError> {
        if let Some(suggestion) = highlighted_suggestion {
            return Ok(Replacement::Some(suggestion));
        }

        // If there's only one clear suggestion, complete to it
        let suggestions = self.get_suggestions(input)?;
        if suggestions.len() == 1 {
            return Ok(Replacement::Some(suggestions[0].clone()));
        }

        Ok(Replacement::None)
    }

    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        use std::{env, fs, path::PathBuf};

        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        let (base_dir, partial_name): (PathBuf, String) = if input.is_empty() {
            (current_dir, String::new())
        } else {
            let input_path = PathBuf::from(input);
            if input_path.is_dir() {
                (input_path, String::new())
            } else {
                let parent = input_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| current_dir.clone());
                let file_name = input_path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                (parent, file_name)
            }
        };

        let mut suggestions: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir(&base_dir) {
            for entry_result in entries {
                if let Ok(entry) = entry_result {
                    let file_name_os = entry.file_name();
                    let file_name = file_name_os.to_string_lossy();
                    if file_name.starts_with(&partial_name) {
                        let full_path = base_dir.join(file_name.as_ref());
                        if !self.directories && full_path.is_dir() {
                            continue; // Skip directories if only files are requested
                        }
                        if !self.files && full_path.is_file() {
                            continue; // Skip files if only directories are requested
                        }
                        let mut display = full_path.to_string_lossy().to_string();
                        if full_path.is_dir() {
                            let sep = std::path::MAIN_SEPARATOR;
                            if !display.ends_with(sep) {
                                display.push(sep);
                            }
                        }
                        suggestions.push(display);
                    }
                }
            }
        }

        // Sort suggestions: case-insensitive, directories and files mixed but stable
        suggestions.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        Ok(suggestions)
    }
}
