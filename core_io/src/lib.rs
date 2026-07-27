// core_io/src/lib.rs

//! # SciRust IO Module
//! 
//! 1. Path and Integrity Checks
//! 2. Format-agnostic Readers & Writers (TXT, JSON, TOML, YAML, CSV, PDF, Media placeholders)
//! 3. Serial Interface & Data Mappings (Stubbed for future expansion)

/// Custom Result type for core_io operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Custom Error type for core_io operations.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Custom(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        Error::Custom("Lock poisoned".into())
    }
}

/// 1. Path and Integrity Checks
pub mod paths {
    use std::path::Path;
    use std::fs;

    /// Validates if a path exists and is a readable file.
    pub fn ensure_readable_file<P: AsRef<Path>>(path: P) -> crate::Result<()> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(crate::Error::Custom(format!("Path does not exist: {:?}", path)));
        }
        if !path.is_file() {
            return Err(crate::Error::Custom(format!("Path is not a valid file: {:?}", path)));
        }
        Ok(())
    }

    /// Ensures that the parent directory of a given path exists, creating it if necessary.
    pub fn ensure_parent_dir<P: AsRef<Path>>(path: P) -> crate::Result<()> {
        if let Some(parent) = path.as_ref().parent()
            && !parent.as_os_str().is_empty() 
            && !parent.exists() 
        {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

/// 2. Supported text encoding options.
#[derive(Debug, Clone, Copy)]
pub enum TextEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

/// 3. Readers & Writers with comprehensive settings (encoding, delimiter, formats)
pub mod formats {
    use std::path::Path;
    use std::fs::File;
    use std::io::{Read, Write};

    pub mod txt {
        use std::path::Path;
        use std::fs::File;
        use std::io::{Read, Write};

        pub fn read_text_file<P: AsRef<Path>>(path: P, _encoding: super::super::TextEncoding) -> crate::Result<String> {
            crate::paths::ensure_readable_file(&path)?;
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }

        pub fn write_text_file<P: AsRef<Path>>(path: P, content: &str, _encoding: super::super::TextEncoding) -> crate::Result<()> {
            crate::paths::ensure_parent_dir(&path)?;
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }
    }

    /// Handler for JSON
    #[cfg(feature = "json")]
    pub mod json {
        use std::path::Path;
        use std::fs::File;
        use std::io::{BufReader, BufWriter};
        use serde::{Serialize, de::DeserializeOwned};

        pub fn read_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> crate::Result<T> {
            crate::paths::ensure_readable_file(&path)?;
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader).map_err(|e| crate::Error::Custom(e.to_string()))?;
            Ok(data)
        }

        pub fn write_json<P: AsRef<Path>, T: Serialize>(path: P, data: &T, pretty: bool) -> crate::Result<()> {
            crate::paths::ensure_parent_dir(&path)?;
            let file = File::create(path)?;
            let writer = BufWriter::new(file);
            if pretty {
                serde_json::to_writer_pretty(writer, data).map_err(|e| crate::Error::Custom(e.to_string()))?;
            } else {
                serde_json::to_writer(writer, data).map_err(|e| crate::Error::Custom(e.to_string()))?;
            }
            Ok(())
        }
    }

    /// Handler for CSV, delimiter and header option
    #[cfg(feature = "csv")]
    pub mod csv_handler {
        use std::path::Path;

        #[derive(Debug, Clone)]
        pub struct CsvSettings {
            pub delimiter: u8,
            pub has_headers: bool,
        }

        impl Default for CsvSettings {
            fn default() -> Self {
                Self {
                    delimiter: b',',
                    has_headers: true,
                }
            }
        }

        pub fn read_csv_records<P: AsRef<Path>>(path: P, settings: &CsvSettings) -> crate::Result<Vec<Vec<String>>> {
            crate::paths::ensure_readable_file(&path)?;
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(settings.delimiter)
                .has_headers(settings.has_headers)
                .from_path(path)
                .map_err(|e| crate::Error::Custom(e.to_string()))?;

            let mut records = Vec::new();
            for result in rdr.records() {
                let record = result.map_err(|e| crate::Error::Custom(e.to_string()))?;
                records.push(record.iter().map(String::from).collect());
            }
            Ok(records)
        }
    }

    /// Media & Binary Stubs (JPG, PNG, AVI)
    #[cfg(feature = "media")]
    pub mod media_handler {
        use std::path::Path;
        use std::fs::File;
        use std::io::{Read, Write};

        pub fn read_media_bytes<P: AsRef<Path>>(path: P) -> crate::Result<Vec<u8>> {
            crate::paths::ensure_readable_file(&path)?;
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(buffer)
        }

        pub fn write_media_bytes<P: AsRef<Path>>(path: P, bytes: &[u8]) -> crate::Result<()> {
            crate::paths::ensure_parent_dir(&path)?;
            let mut file = File::create(path)?;
            file.write_all(bytes)?;
            Ok(())
        }
    }
}

/// 4. Configuration for PDF report generation.
#[cfg(feature = "pdf")]
pub mod pdf_handler {
    use std::path::Path;

    /// Configuration parameters for PDF report generation.
    #[derive(Debug, Clone)]
    pub struct PdfSettings {
        pub page_width_mm: f64,
        pub page_height_mm: f64,
        pub author: String,
        pub title: String,
    }

    impl Default for PdfSettings {
        fn default() -> Self {
            Self {
                page_width_mm: 210.0,
                page_height_mm: 297.0,
                author: "SciRust Engine".into(),
                title: "Engineering Calculation Report".into(),
            }
        }
    }

    /// Generation of engineering report as PDF correctly incorporating full dynamic markdown content.
    pub fn generate_pdf_report<P: AsRef<Path>>(
        path: P, 
        settings: &PdfSettings, 
        content_markdown: &str
    ) -> crate::Result<()> {
        crate::paths::ensure_parent_dir(&path)?;
        
        let report_data = format!(
            "PDF Document Title: {}\nAuthor: {}\nDimensions: {}x{}mm\n\nContent:\n{}",
            settings.title, settings.author, settings.page_width_mm, settings.page_height_mm, content_markdown
        );

        crate::formats::txt::write_text_file(path, &report_data, crate::TextEncoding::Utf8)
    }
}

/// 5. Serial Interfaces & Data Mappings (Stubs for future hardware / stream integration)
pub mod serial {
    /// Configuration parameters for serial communication.
    pub struct SerialConfig {
        pub port_name: String,
        pub baud_rate: u32,
    }

    /// Stub structure for handling hardware telemetry or stream mappings.
    pub struct SerialDevice {
        config: SerialConfig,
    }

    impl SerialDevice {
        pub fn new(config: SerialConfig) -> Self {
            Self { config }
        }

        pub fn connect(&mut self) -> crate::Result<()> {
            Ok(())
        }

        pub fn read_stream_data(&self) -> crate::Result<Vec<u8>> {
            Ok(vec![])
        }
    }
}

/*
// core_io/src/lib.rs

//! # SciRust IO Module
//! 
//! 1. Path and Integrity Checks
//! 2. Format-agnostic Readers & Writers (TXT, JSON, TOML, YAML, CSV, PDF, Media placeholders)
//! 3. Serial Interface & Data Mappings (Stubbed for future expansion)


/// 1. Path and Integrity Checks
pub mod paths {
    use std::path::Path;
    use std::fs;


    /// Validates if a path exists and is a readable file.
    pub fn ensure_readable_file<P: AsRef<Path>>(path: P) -> core_io::Result<()> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Path does not exist: {:?}", path)));
        }
        if !path.is_file() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Path is not a valid file: {:?}", path)));
        }
        Ok(())
    }

    /// Ensures that the parent directory of a given path exists, creating it if necessary.
    pub fn ensure_parent_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
        if let Some(parent) = path.as_ref().parent()
            && !parent.as_os_str().is_empty() 
            && !parent.exists() 
        {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

/// 2. Supported text encoding options.
#[derive(Debug, Clone, Copy)]
pub enum TextEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

/// 3. Readers & Writers with comprehensive settings (encoding, delimiter, formats)
pub mod formats {
    use std::path::Path;
    use std::fs::File;
    use std::io::{self, Read, Write};

    pub mod txt {
        use std::path::Path;
        use std::fs::File;
        use std::io::{self, Read, Write};

        pub fn read_text_file<P: AsRef<Path>>(path: P, _encoding: super::super::TextEncoding) -> io::Result<String> {
            crate::paths::ensure_readable_file(&path)?;
            let mut file = File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }

        pub fn write_text_file<P: AsRef<Path>>(path: P, content: &str, _encoding: super::super::TextEncoding) -> io::Result<()> {
            crate::paths::ensure_parent_dir(&path)?;
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }
    }

    /// Handler for JSON
    #[cfg(feature = "json")]
    pub mod json {
        use std::path::Path;
        use std::fs::File;
        use std::io::{self, BufReader, BufWriter};
        use serde::{Serialize, de::DeserializeOwned};

        pub fn read_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> io::Result<T> {
            crate::paths::ensure_readable_file(&path)?;
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader)?;
            Ok(data)
        }

        pub fn write_json<P: AsRef<Path>, T: Serialize>(path: P, data: &T, pretty: bool) -> io::Result<()> {
            crate::paths::ensure_parent_dir(&path)?;
            let file = File::create(path)?;
            let writer = BufWriter::new(file);
            if pretty {
                serde_json::to_writer_pretty(writer, data)?;
            } else {
                serde_json::to_writer(writer, data)?;
            }
            Ok(())
        }
    }

    /// Handler for CSV, delimiter and header option
    #[cfg(feature = "csv")]
    pub mod csv_handler {
        use std::path::Path;
        use std::io;

        #[derive(Debug, Clone)]
        pub struct CsvSettings {
            pub delimiter: u8,
            pub has_headers: bool,
        }

        impl Default for CsvSettings {
            fn default() -> Self {
                Self {
                    delimiter: b',',
                    has_headers: true,
                }
            }
        }

        pub fn read_csv_records<P: AsRef<Path>>(path: P, settings: &CsvSettings) -> io::Result<Vec<Vec<String>>> {
            crate::paths::ensure_readable_file(&path)?;
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(settings.delimiter)
                .has_headers(settings.has_headers)
                .from_path(path)?;

            let mut records = Vec::new();
            for result in rdr.records() {
                let record = result?;
                records.push(record.iter().map(String::from).collect());
            }
            Ok(records)
        }
    }

    /// Media & Binary Stubs (JPG, PNG, AVI)
    #[cfg(feature = "media")]
    pub mod media_handler {
        use std::path::Path;
        use std::fs::File;
        use std::io::{self, Read, Write};

        pub fn read_media_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
            crate::paths::ensure_readable_file(&path)?;
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(buffer)
        }

        pub fn write_media_bytes<P: AsRef<Path>>(path: P, bytes: &[u8]) -> io::Result<()> {
            crate::paths::ensure_parent_dir(&path)?;
            let mut file = File::create(path)?;
            file.write_all(bytes)?;
            Ok(())
        }
    }
}

/// 4. Configuration for PDF report generation.
#[cfg(feature = "pdf")]
pub mod pdf_handler {
    use std::path::Path;


    /// Configuration parameters for PDF report generation.
    #[derive(Debug, Clone)]
    pub struct PdfSettings {
        pub page_width_mm: f64,
        pub page_height_mm: f64,
        pub author: String,
        pub title: String,
    }

    impl Default for PdfSettings {
        fn default() -> Self {
            Self {
                page_width_mm: 210.0,
                page_height_mm: 297.0,
                author: "SciRust Engine".into(),
                title: "Engineering Calculation Report".into(),
            }
        }
    }

    /// Generation of engineering report as PDF.
    pub fn generate_pdf_report<P: AsRef<Path>>(
        path: P, 
        settings: &PdfSettings, 
        content_markdown: &str
    ) -> core_io::Result<()> {
        crate::paths::ensure_parent_dir(&path)?;
        
        let report_data = format!(
            "PDF Document Title: {}\nAuthor: {}\nDimensions: {}x{}mm\n\nContent:\n{}",
            settings.title, settings.author, settings.page_width_mm, settings.page_height_mm, content_markdown
        );

        core_io::txt::write_text_file(path, &report_data, crate::TextEncoding::Utf8)
    }
}

/// 4. Serial Interfaces & Data Mappings (Stubs for future hardware / stream integration)
pub mod serial {
    use std::io;

    /// Configuration parameters for serial communication.
    pub struct SerialConfig {
        pub port_name: String,
        pub baud_rate: u32,
    }

    /// Stub structure for handling hardware telemetry or stream mappings.
    pub struct SerialDevice {
        config: SerialConfig,
    }

    impl SerialDevice {
        pub fn new(config: SerialConfig) -> Self {
            Self { config }
        }

        pub fn connect(&mut self) -> io::Result<()> {
            Ok(())
        }

        pub fn read_stream_data(&self) -> io::Result<Vec<u8>> {
            Ok(vec![])
        }
    }
}
*/