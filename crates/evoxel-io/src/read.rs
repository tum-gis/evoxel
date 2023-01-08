use crate::document::VoxelGridInfoDocument;
use crate::error::Error;
use evoxel_core::voxel_grid::{VoxelGrid, VoxelGridInfo};
use polars::prelude::{all, LazyCsvReader};
use std::fs;
use std::path::{Path, PathBuf};

/// `EvoxelReader` sets up a reader for the custom reader data structure.
///
#[derive(Clone)]
pub struct EvoxelReader {
    path: PathBuf,
}

impl EvoxelReader {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }

    pub fn finish(self) -> Result<VoxelGrid, Error> {
        // assert!(self.path.is_dir());
        // assert!(self.path.exists(), "Path must exist.");

        let voxel_grid_data_path = self.path.join("voxel_grid.xyz");
        let df = LazyCsvReader::new(voxel_grid_data_path)
            .with_delimiter(b' ')
            .finish()?
            .select([all()])
            .collect()?;

        // let vg = VoxelGrid
        let voxel_grid_info_path = self.path.join("info.json");
        let voxel_grid_info_string =
            fs::read_to_string(voxel_grid_info_path).expect("Unable to read file");
        let voxel_grid_document: VoxelGridInfoDocument =
            serde_json::from_str(&voxel_grid_info_string).expect("Unable to parse file");
        let voxel_grid_info = VoxelGridInfo::new(
            voxel_grid_document.frame_id,
            voxel_grid_document.resolution,
            voxel_grid_document.start_timestamp.into(),
            voxel_grid_document.stop_timestamp.into(),
            voxel_grid_document.submap_index,
        );

        let frames_path = self.path.join("frames.json");
        let frames = ecoord::io::EcoordReader::new(frames_path).finish()?;

        let voxel_grid = VoxelGrid::new(df, voxel_grid_info, frames);
        Ok(voxel_grid)
    }
}
