use crate::Error;
use evoxel_core::{VoxelDataColumnNames, VoxelGrid};
use nalgebra::Point3;

use polars::frame::DataFrame;

use crate::Error::LowerCornerMustBeBelowUpperCorner;
use polars::prelude::{all, col, len, ChunkCompare, IntoLazy};

pub fn aggregate_by_index(voxel_grid: &VoxelGrid) -> Result<VoxelGrid, Error> {
    let voxel_data = voxel_grid.voxel_data();
    let partition_columns = vec![
        VoxelDataColumnNames::X.as_str(),
        VoxelDataColumnNames::Y.as_str(),
        VoxelDataColumnNames::Z.as_str(),
    ];

    let partitioned: DataFrame = voxel_data
        .clone()
        .lazy()
        .group_by(partition_columns)
        .agg([all(), len()])
        //.limit(5)
        .collect()?;
    //dbg!("{:?}", &partitioned);

    /*println!("partitions: {}", partitioned.len());
    let parit_filtered: Vec<DataFrame> = partitioned
        .into_iter()
        .filter(|d| d.height() >= minimum)
        .collect();

    let partitioned_lazy: Vec<LazyFrame> =
        parit_filtered.iter().map(|d| d.clone().lazy()).collect();

    let merged_again = concat(partitioned_lazy, true, true)
        .unwrap()
        .collect()
        .unwrap();*/

    let filtered_voxel_grid = VoxelGrid::new(
        partitioned,
        voxel_grid.info().clone(),
        voxel_grid.reference_frames().clone(),
    )?;
    Ok(filtered_voxel_grid)
}

pub fn explode(voxel_grid: &VoxelGrid) -> Result<VoxelGrid, Error> {
    let voxel_data = voxel_grid.voxel_data();

    let column_names: Vec<&str> = voxel_data
        .get_columns()
        .iter()
        .filter(|s| s.dtype().inner_dtype().is_some()) // if contains inner, it's a list
        .map(|s| s.name())
        .collect();

    let df: DataFrame = voxel_data
        .clone()
        .lazy()
        .explode(column_names.into_iter().map(col).collect::<Vec<_>>())
        //.limit(5)
        .collect()?;

    let filtered_voxel_grid = VoxelGrid::new(
        df,
        voxel_grid.info().clone(),
        voxel_grid.reference_frames().clone(),
    )?;
    Ok(filtered_voxel_grid)
}

pub fn filter_by_count(voxel_grid: &VoxelGrid, minimum: usize) -> Result<VoxelGrid, Error> {
    let voxel_data = voxel_grid.voxel_data().clone();

    let mask = voxel_data
        .column(VoxelDataColumnNames::Count.as_str())?
        .gt_eq(minimum as i32)?;

    let filtered_voxel_data = voxel_data.filter(&mask)?;
    let filtered_voxel_grid = VoxelGrid::new(
        filtered_voxel_data,
        voxel_grid.info().clone(),
        voxel_grid.reference_frames().clone(),
    )?;

    Ok(filtered_voxel_grid)
}

pub fn filter_by_index_bounds(
    voxel_grid: &VoxelGrid,
    lower_corner: Point3<i64>,
    upper_corner: Point3<i64>,
) -> Result<VoxelGrid, Error> {
    if lower_corner >= upper_corner {
        return Err(LowerCornerMustBeBelowUpperCorner(""));
    }

    let voxel_data = voxel_grid.voxel_data().clone();

    let filtered_voxel_data = voxel_data
        .lazy()
        .filter(
            col(VoxelDataColumnNames::X.as_str())
                .gt_eq(lower_corner.x)
                .and(col(VoxelDataColumnNames::X.as_str()).lt_eq(upper_corner.x)),
        )
        .filter(
            col(VoxelDataColumnNames::Y.as_str())
                .gt_eq(lower_corner.y)
                .and(col(VoxelDataColumnNames::Y.as_str()).lt_eq(upper_corner.y)),
        )
        .filter(
            col(VoxelDataColumnNames::Z.as_str())
                .gt_eq(lower_corner.z)
                .and(col(VoxelDataColumnNames::Z.as_str()).lt_eq(upper_corner.z)),
        )
        .collect()?;
    let filtered_voxel_grid = VoxelGrid::new(
        filtered_voxel_data,
        voxel_grid.info().clone(),
        voxel_grid.reference_frames().clone(),
    )?;

    Ok(filtered_voxel_grid)
}
