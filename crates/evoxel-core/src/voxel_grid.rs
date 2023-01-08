use chrono::{DateTime, Utc};
use ecoord::{FrameId, ReferenceFrames, TransformId};
use nalgebra::Point3;
use polars::datatypes::DataType;
use polars::prelude::{DataFrame, TakeRandom};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct VoxelGrid {
    pub data: DataFrame,
    pub info: VoxelGridInfo,
    pub frames: ReferenceFrames,
}

impl VoxelGrid {
    pub fn new(data: DataFrame, info: VoxelGridInfo, frames: ReferenceFrames) -> Self {
        let column_names = data.get_column_names();
        assert_eq!(column_names[0], "x");
        assert_eq!(column_names[1], "y");
        assert_eq!(column_names[2], "z");

        let data_types = data.dtypes();
        assert_eq!(data_types[0], DataType::Int64);
        assert_eq!(data_types[1], DataType::Int64);
        assert_eq!(data_types[2], DataType::Int64);

        Self { data, info, frames }
    }

    pub fn size(&self) -> usize {
        self.data.height()
    }

    pub fn min_index(&self) -> Point3<i64> {
        let selected_df_row = self.data.min();
        let index_x: i64 = selected_df_row
            .column(DataFrameColumnNames::X.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .unwrap();
        let index_y: i64 = selected_df_row
            .column(DataFrameColumnNames::Y.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .unwrap();
        let index_z: i64 = selected_df_row
            .column(DataFrameColumnNames::Z.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .unwrap();

        Point3::new(index_x, index_y, index_z)
    }

    pub fn max_index(&self) -> Point3<i64> {
        let selected_df_row = self.data.max();
        let index_x: i64 = selected_df_row
            .column(DataFrameColumnNames::X.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .unwrap();
        let index_y: i64 = selected_df_row
            .column(DataFrameColumnNames::Y.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .unwrap();
        let index_z: i64 = selected_df_row
            .column(DataFrameColumnNames::Z.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .unwrap();

        Point3::new(index_x, index_y, index_z)
    }

    /// Returns all cell indices as a vector in the local coordinate frame.
    ///
    ///
    pub fn get_all_cell_indices_in_local_frame(&self) -> Vec<Point3<i64>> {
        let x_series = self
            .data
            .column(DataFrameColumnNames::X.as_str())
            .unwrap()
            .i64()
            .unwrap();
        let y_series = self
            .data
            .column(DataFrameColumnNames::Y.as_str())
            .unwrap()
            .i64()
            .unwrap();
        let z_series = self
            .data
            .column(DataFrameColumnNames::Z.as_str())
            .unwrap()
            .i64()
            .unwrap();

        let all_indices: Vec<Point3<i64>> = (0..self.size() as usize)
            .into_par_iter()
            .map(|i: usize| {
                Point3::new(
                    x_series.get(i).unwrap(),
                    y_series.get(i).unwrap(),
                    z_series.get(i).unwrap(),
                )
            })
            .collect();

        all_indices
    }

    pub fn get_all_center_points_in_local_frame(&self) -> Vec<Point3<f64>> {
        let all_indices = self.get_all_cell_indices_in_local_frame();
        let all_center_points = all_indices
            .par_iter()
            .map(|c| {
                Point3::new(
                    self.info.resolution * c.x as f64,
                    self.info.resolution * c.y as f64,
                    self.info.resolution * c.z as f64,
                )
            })
            .collect();

        all_center_points
    }

    pub fn get_cell_index(&self, row_index: usize) -> Point3<i64> {
        let index_x: i64 = self
            .data
            .column(DataFrameColumnNames::X.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(row_index)
            .unwrap();
        let index_y: i64 = self
            .data
            .column(DataFrameColumnNames::Y.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(row_index)
            .unwrap();
        let index_z: i64 = self
            .data
            .column(DataFrameColumnNames::Z.as_str())
            .unwrap()
            .i64()
            .unwrap()
            .get(row_index)
            .unwrap();

        Point3::new(index_x, index_y, index_z)
    }

    /// Returns the frame id of the voxel grid coordinates
    ///
    pub fn get_local_frame_id(&self) -> FrameId {
        //let t: &Transformation = self.frames.transforms().get();
        FrameId::from(self.info.frame_id.clone())
    }

    pub fn get_local_center_point(&self, row_idx: usize) -> Point3<f64> {
        let index: Point3<i64> = self.get_cell_index(row_idx);
        let x: f64 = self.info.resolution * index.x as f64;
        let y: f64 = self.info.resolution * index.y as f64;
        let z: f64 = self.info.resolution * index.z as f64;

        Point3::new(x, y, z)
    }

    pub fn get_center_point(
        &self,
        row_idx: usize,
        frame_id: &FrameId,
        timestamp: DateTime<Utc>,
    ) -> Point3<f64> {
        let local_center_point = self.get_local_center_point(row_idx);

        let transform_id = TransformId::new(frame_id.clone(), self.get_local_frame_id());

        let isometry = self
            .frames
            .derive_transform_graph(&None, &Some(timestamp))
            .get_isometry(&transform_id);

        // let isometry = self.pose.isometry();
        isometry * local_center_point
    }

    pub fn get_all_center_points(
        &self,
        frame_id: &FrameId,
        timestamp: DateTime<Utc>,
    ) -> Vec<Point3<f64>> {
        let a: Vec<Point3<f64>> = (0..self.size() as i64)
            .into_iter()
            .map(|i: i64| self.get_center_point(i as usize, frame_id, timestamp))
            .collect();
        a
    }
}

#[derive(Debug, Clone)]
pub struct VoxelGridInfo {
    pub frame_id: String,
    pub resolution: f64,
    pub start_time: DateTime<Utc>,
    pub stop_time: DateTime<Utc>,
    pub submap_index: i32,
}

impl VoxelGridInfo {
    pub fn new(
        frame_id: String,
        resolution: f64,
        start_time: DateTime<Utc>,
        stop_time: DateTime<Utc>,
        submap_index: i32,
    ) -> Self {
        Self {
            frame_id,
            resolution,
            start_time,
            stop_time,
            submap_index,
        }
    }
}

enum DataFrameColumnNames {
    X,
    Y,
    Z,
}

impl DataFrameColumnNames {
    fn as_str(&self) -> &'static str {
        match self {
            DataFrameColumnNames::X => "x",
            DataFrameColumnNames::Y => "y",
            DataFrameColumnNames::Z => "z",
        }
    }
}
