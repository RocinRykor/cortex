use crate::primitives::files::base_file::BaseFile;

pub struct LzmaFile {
    pub base: BaseFile,
    pub compression_level: u32, // lzma-specific
}