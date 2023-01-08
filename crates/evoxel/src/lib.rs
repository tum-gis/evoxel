//! Library for representing voxel grids with the extension `.evox`.
//!
//! References:
//!
//! - [parry3d_f64::transformation::voxelization](https://docs.rs/parry3d-f64/latest/parry3d_f64/transformation/voxelization/index.html)
//! - [octomap](https://github.com/OctoMap/octomap)
//!
//! # Overview
//!
//!
//! # Data structure
//!
//! For serializing a voxel grid, this data structure is used:
//!
//! - `voxel_grid_name` (directory) or `voxel_grid_name.evox` (single file as [tarball](https://en.wikipedia.org/wiki/Tar_(computing)))
//!     - `voxel_grid.xyz` (uncompressed) or `voxel_grid.parquet` (compressed)
//!         - mandatory fields:
//!             - `x`: [i64](i64)
//!             - `y`: [i64](i64)
//!             - `z`: [i64](i64)
//!     - `info.json`
//!         - mandatory fields:
//!             - `frame_id`: [String](String)
//!             - `resolution`: [f64](f64)
//!             - `start_time`: [i128](i128)
//!             - `stop_time`: [i128](i128)
//!             - `submap_index`: [i32](i32)
//!     - `frames.json`
//!         - contains a transformation tree with validity durations
//!         - kind of similar to a geopose
//!         - information: srid
//!         - purpose: translate and rotate the voxel grid without reading/writing the point data
//!
//!     - `preview.geojson`
//!         - bounding box
//!         - purpose: fast visualization
//!

pub use evoxel_core::{VoxelGrid, VoxelGridInfo};

pub use evoxel_io as io;
