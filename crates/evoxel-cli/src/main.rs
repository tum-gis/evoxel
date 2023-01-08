use evoxel::io::EvoxelReader;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Hello, world!");

    let path = PathBuf::from("/submap_0");
    let start = Instant::now();
    let voxel_grid = EvoxelReader::new(path).finish().unwrap();
    let duration = start.elapsed();
    info!(
        "Read voxel grid with {} cells in {:?}.",
        voxel_grid.size(),
        duration
    );

    info!("Start");
    let start = Instant::now();
    let c = voxel_grid.get_all_cell_indices_in_local_frame();
    let duration = start.elapsed();
    info!("Calculated {} points in {:?}.", c.len(), duration);
    //let all = voxel_grid.get_all_center_points();
}
