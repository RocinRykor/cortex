use crate::primitives::files::base_file::BaseFile;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub struct JsonFile {
    pub base: BaseFile,
    pub pretty_print: bool, // json-specific: should output be formatted?
}

impl JsonFile {
    pub fn new(title: String, data: Vec<u8>) -> Self {
        Self {
            base: BaseFile {
                title,
                extension: "json".to_string(),
                data,
            },
            pretty_print: false,
        }
    }

    /// Borrow the raw content as a str without cloning
    pub fn content(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.base.data)
    }

    /// Check if the contents are valid JSON
    pub fn is_valid_json(&self) -> bool {
        self.content()
            .map(|s| serde_json::from_str::<Value>(s).is_ok())
            .unwrap_or(false)
    }

    /// Parse into a generic JSON value tree
    pub fn parse(&self) -> serde_json::Result<Value> {
        let s = self.content().map_err(|e| {
            serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })?;
        serde_json::from_str(s)
    }

    /// Parse directly into a concrete type
    pub fn deserialize<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        let s = self.content().map_err(|e| {
            serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })?;
        serde_json::from_str(s)
    }

    /// Serialize a value back into this file's data buffer
    pub fn write<T: serde::Serialize>(&mut self, value: &T) -> serde_json::Result<()> {
        self.base.data = if self.pretty_print {
            serde_json::to_vec_pretty(value)?
        } else {
            serde_json::to_vec(value)?
        };
        Ok(())
    }
}
