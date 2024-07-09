use crate::document::EvoxelInfoDocument;
use crate::error::Error;
use crate::write_impl::{write_to_parquet, write_to_xyz};
use crate::{
    FILE_NAME_ECOORD, FILE_NAME_INFO, FILE_NAME_VOXEL_DATA_COMPRESSED,
    FILE_NAME_VOXEL_DATA_UNCOMPRESSED,
};
use evoxel_core::VoxelGrid;
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

/// `EvoxelReader` sets up a reader for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EvoxelWriter {
    path: PathBuf,
    compressed: bool,
}

impl EvoxelWriter {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            compressed: true,
        }
    }

    pub fn with_compressed(mut self, compressed: bool) -> Self {
        self.compressed = compressed;
        self
    }

    pub fn finish(&self, voxel_grid: &VoxelGrid) -> Result<(), Error> {
        fs::create_dir_all(self.path.clone())?;

        if self.compressed {
            let parquet_file_path = self.path.join(FILE_NAME_VOXEL_DATA_COMPRESSED);
            write_to_parquet(voxel_grid, parquet_file_path)?;
        } else {
            let xyz_file_path = self.path.join(FILE_NAME_VOXEL_DATA_UNCOMPRESSED);
            write_to_xyz(voxel_grid, xyz_file_path)?;
        }

        let info_document_path = self.path.join(FILE_NAME_INFO);
        let info_document = EvoxelInfoDocument::new(
            voxel_grid.info().frame_id().clone().into(),
            voxel_grid.info().resolution(),
        );
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(info_document_path)?;
        serde_json::to_writer_pretty(file, &info_document)?;

        let ecoord_document_path = self.path.join(FILE_NAME_ECOORD);
        ecoord::io::EcoordWriter::from_path(ecoord_document_path)?
            .finish(voxel_grid.reference_frames())?;

        Ok(())
    }
}
