//! `evoxel` is a library for processing 3D voxel grids.
//!
//!
//! # Overview
//!
//!
//! # Data structure
//!
//! For serializing a voxel grid, this data structure is used:
//!
//! - `voxel_grid_name` (directory) or `voxel_grid_name.evoxel` (single file as [tarball](https://en.wikipedia.org/wiki/Tar_(computing)))
//!     - `voxel_data.xyz` (uncompressed) or `voxel_data.parquet` (compressed)
//!         - mandatory fields:
//!             - `x` [i64]: X index
//!             - `y` [i64]: Y index
//!             - `z` [i64]: Z index
//!     - `info.json`
//!         - mandatory fields:
//!             - `frame_id` [String]
//!             - `resolution` [f64]
//!         - optional fields:
//!             - `start_time` [i128]
//!             - `stop_time` [i128]
//!             - `submap_index` [i32]
//!     - `ecoord.json`
//!         - contains a transformation tree with validity durations
//!         - information: srid
//!         - purpose: translate and rotate the voxel grid without reading/writing the point data
//!
//! # Other projects
//!
//! - [parry3d_f64::transformation::voxelization](https://docs.rs/parry3d-f64/latest/parry3d_f64/transformation/voxelization/index.html)
//! - [octomap](https://github.com/OctoMap/octomap)

pub use evoxel_core::{Error, VoxelDataColumnNames, VoxelGrid, VoxelGridInfo};

pub use evoxel_io as io;

pub use evoxel_transform as transform;
