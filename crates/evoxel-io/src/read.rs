use crate::document::EvoxelInfoDocument;
use crate::error::Error;
use crate::{FILE_NAME_ECOORD, FILE_NAME_INFO, FILE_NAME_VOXEL_DATA_UNCOMPRESSED};
use evoxel_core::voxel_grid::VoxelGrid;
use evoxel_core::VoxelGridInfo;
use polars::prelude::LazyFileListReader;
use polars::prelude::{all, LazyCsvReader};
use std::fs;
use std::path::{Path, PathBuf};

/// `EvoxelReader` sets up a reader for the custom reader data structure.
///
#[derive(Debug, Clone)]
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

        let voxel_grid_data_path = self.path.join(FILE_NAME_VOXEL_DATA_UNCOMPRESSED);
        let df = LazyCsvReader::new(voxel_grid_data_path)
            .with_separator(b' ')
            .finish()?
            .select([all()])
            .collect()?;

        // let vg = VoxelGrid
        let voxel_grid_info_path = self.path.join(FILE_NAME_INFO);
        let voxel_grid_info_string =
            fs::read_to_string(voxel_grid_info_path).expect("Unable to read file");
        let voxel_grid_document: EvoxelInfoDocument =
            serde_json::from_str(&voxel_grid_info_string).expect("Unable to parse file");
        let voxel_grid_info = VoxelGridInfo::new(
            voxel_grid_document.frame_id.into(),
            voxel_grid_document.resolution,
            voxel_grid_document.start_timestamp.map(|t| t.into()),
            voxel_grid_document.stop_timestamp.map(|t| t.into()),
            voxel_grid_document.submap_index,
        );

        let ecoord_path = self.path.join(FILE_NAME_ECOORD);
        let reference_frames = ecoord::io::EcoordReader::from_path(ecoord_path)?.finish()?;

        let voxel_grid = VoxelGrid::new(df, voxel_grid_info, reference_frames)?;
        Ok(voxel_grid)
    }
}
