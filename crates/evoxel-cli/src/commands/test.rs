use evoxel::io::{EvoxelReader, EvoxelWriter};
use nalgebra::Point3;
use std::path::Path;
use std::time::Instant;
use tracing::info;

pub fn run(input_directory_path: impl AsRef<Path>, output_directory_path: impl AsRef<Path>) {
    // let path = PathBuf::from("/submap_0");
    let start = Instant::now();
    let voxel_grid = EvoxelReader::new(input_directory_path).finish().unwrap();
    let duration = start.elapsed();
    info!(
        "Read voxel grid with {} cells in {:?}.",
        voxel_grid.size(),
        duration
    );

    let voxel_grid = evoxel::transform::aggregate_by_index(&voxel_grid).unwrap();
    let voxel_grid = evoxel::transform::filter_by_count(&voxel_grid, 3).unwrap();
    let voxel_grid = evoxel::transform::explode(&voxel_grid).unwrap();
    let voxel_grid = evoxel::transform::filter_by_index_bounds(
        &voxel_grid,
        Point3::new(676, 95, 0),
        Point3::new(1271, 135, 86),
    )
    .unwrap();

    info!("Start");
    let start = Instant::now();
    let c = voxel_grid.get_all_cell_indices_in_local_frame();
    let duration = start.elapsed();
    info!("Calculated {} points in {:?}.", c.len(), duration);
    //let all = voxel_grid.get_all_center_points();

    info!(
        "Start writing voxel grid with {} cells to {}",
        voxel_grid.size(),
        output_directory_path.as_ref().display()
    );
    EvoxelWriter::new(output_directory_path)
        .with_compressed(false)
        .finish(&voxel_grid)
        .expect("should work");
}
