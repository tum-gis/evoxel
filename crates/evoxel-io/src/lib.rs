mod document;
mod error;
mod read;
mod write;
mod write_impl;

#[doc(inline)]
pub use crate::error::Error;

#[doc(inline)]
pub use crate::read::EvoxelReader;

#[doc(inline)]
pub use crate::write::EvoxelWriter;

pub const FILE_NAME_VOXEL_DATA_COMPRESSED: &str = "voxel_data.parquet";
pub const FILE_NAME_VOXEL_DATA_UNCOMPRESSED: &str = "voxel_data.xyz";
pub const FILE_NAME_INFO: &str = "info.json";
pub const FILE_NAME_ECOORD: &str = "ecoord.json";
