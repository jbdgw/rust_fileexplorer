use serde::{Deserialize, Serialize};

/// Smart file categorization based on heuristics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileCategory {
    /// Source code files
    Source { language: String },
    /// Build artifacts and compiled binaries
    Build,
    /// Configuration files
    Config { format: String },
    /// Documentation files
    Documentation,
    /// Media files (images, videos, audio)
    Media { media_type: MediaType },
    /// Data files (CSV, JSON, XML, databases)
    Data { format: String },
    /// Compressed archives
    Archive,
    /// Executable binaries
    Executable,
    /// Unknown or uncategorized
    Unknown,
}

/// Media file types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

impl FileCategory {
    /// Categorize a file based on its extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            // Source code
            "rs" => FileCategory::Source {
                language: "rust".to_string(),
            },
            "go" => FileCategory::Source {
                language: "go".to_string(),
            },
            "py" => FileCategory::Source {
                language: "python".to_string(),
            },
            "js" | "jsx" | "ts" | "tsx" => FileCategory::Source {
                language: "javascript".to_string(),
            },
            "java" => FileCategory::Source {
                language: "java".to_string(),
            },
            "c" | "h" => FileCategory::Source {
                language: "c".to_string(),
            },
            "cpp" | "cc" | "cxx" | "hpp" => FileCategory::Source {
                language: "cpp".to_string(),
            },
            "rb" => FileCategory::Source {
                language: "ruby".to_string(),
            },
            "php" => FileCategory::Source {
                language: "php".to_string(),
            },
            "swift" => FileCategory::Source {
                language: "swift".to_string(),
            },
            "kt" | "kts" => FileCategory::Source {
                language: "kotlin".to_string(),
            },

            // Build artifacts
            "o" | "so" | "dylib" | "dll" | "a" | "lib" => FileCategory::Build,

            // Config
            "toml" => FileCategory::Config {
                format: "toml".to_string(),
            },
            "yaml" | "yml" => FileCategory::Config {
                format: "yaml".to_string(),
            },
            "json" => FileCategory::Config {
                format: "json".to_string(),
            },
            "ini" | "conf" | "cfg" => FileCategory::Config {
                format: "ini".to_string(),
            },

            // Documentation
            "md" | "markdown" => FileCategory::Documentation,
            "txt" | "rst" | "adoc" => FileCategory::Documentation,
            "pdf" | "tex" => FileCategory::Documentation,

            // Media - Images
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "ico" => {
                FileCategory::Media {
                    media_type: MediaType::Image,
                }
            }

            // Media - Video
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" => FileCategory::Media {
                media_type: MediaType::Video,
            },

            // Media - Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" => FileCategory::Media {
                media_type: MediaType::Audio,
            },

            // Data
            "csv" | "tsv" => FileCategory::Data {
                format: "csv".to_string(),
            },
            "xml" => FileCategory::Data {
                format: "xml".to_string(),
            },
            "db" | "sqlite" | "sqlite3" => FileCategory::Data {
                format: "sqlite".to_string(),
            },
            "parquet" => FileCategory::Data {
                format: "parquet".to_string(),
            },

            // Archives
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => FileCategory::Archive,

            // Executables (Unix)
            "sh" | "bash" | "zsh" | "fish" => FileCategory::Executable,

            _ => FileCategory::Unknown,
        }
    }
}
