#[derive(Debug)]
pub enum Format {
    Unknown,
    Raw,
    Json,
    Toml,
    Lzma,
}

pub struct File {
    pub title: String,
    pub extension: String,
    pub data: Vec<u8>,
    pub format: Format,
}

impl Format {
    pub fn valid_extensions(&self) -> &'static [&'static str] {
        match self {
            Format::Lzma => &["lzma", "xz"],
            Format::Json => &["json"],
            Format::Toml => &["toml"],
            Format::Raw | Format::Unknown => &[],
        }
    }

    pub fn default_extension(&self) -> &'static str {
        match self {
            Format::Lzma => "lzma",
            Format::Json => "json",
            Format::Toml => "toml",
            _ => "",
        }
    }
}