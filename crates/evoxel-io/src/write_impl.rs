use crate::error::Error;
use evoxel_core::VoxelGrid;
use polars::io::SerWriter;
use polars::prelude::{CsvWriter, ParquetWriter, StatisticsOptions};
use std::fs::OpenOptions;
use std::path::Path;

pub fn write_to_xyz(voxel_grid: &VoxelGrid, file_path: impl AsRef<Path>) -> Result<(), Error> {
    // fs::remove_file(file_path).expect("File delete failed");

    let mut voxel_data = voxel_grid.voxel_data().clone();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    CsvWriter::new(file)
        .with_separator(b' ')
        .finish(&mut voxel_data)?;

    Ok(())
}

pub fn write_to_parquet(voxel_grid: &VoxelGrid, file_path: impl AsRef<Path>) -> Result<(), Error> {
    let mut voxel_grid = voxel_grid.voxel_data().clone();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;

    ParquetWriter::new(file)
        .with_statistics(StatisticsOptions::default())
        .finish(&mut voxel_grid)?;

    Ok(())
}
