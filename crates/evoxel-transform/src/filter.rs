use crate::Error;
use evoxel_core::{VoxelDataColumnType, VoxelGrid};
use nalgebra::Point3;
use polars::datatypes::PlSmallStr;
use polars::frame::DataFrame;

use crate::Error::LowerCornerMustBeBelowUpperCorner;
use polars::prelude::{ChunkCompareIneq, IntoLazy, Selector, all, col, len};

pub fn aggregate_by_index(voxel_grid: &VoxelGrid) -> Result<VoxelGrid, Error> {
    let voxel_data = voxel_grid.voxel_data();
    let partition_columns = vec![
        VoxelDataColumnType::X.as_str(),
        VoxelDataColumnType::Y.as_str(),
        VoxelDataColumnType::Z.as_str(),
    ];

    let partitioned: DataFrame = voxel_data
        .clone()
        .lazy()
        .group_by(partition_columns)
        .agg([all().as_expr(), len()])
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
        .map(|s| s.name().as_str())
        .collect();

    let df: DataFrame = voxel_data
        .clone()
        .lazy()
        .explode(Selector::ByName {
            names: column_names.into_iter().map(PlSmallStr::from_str).collect(),
            strict: false,
        })
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
        .column(VoxelDataColumnType::Count.as_str())?
        .as_materialized_series()
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
            col(VoxelDataColumnType::X.as_str())
                .gt_eq(lower_corner.x)
                .and(col(VoxelDataColumnType::X.as_str()).lt_eq(upper_corner.x)),
        )
        .filter(
            col(VoxelDataColumnType::Y.as_str())
                .gt_eq(lower_corner.y)
                .and(col(VoxelDataColumnType::Y.as_str()).lt_eq(upper_corner.y)),
        )
        .filter(
            col(VoxelDataColumnType::Z.as_str())
                .gt_eq(lower_corner.z)
                .and(col(VoxelDataColumnType::Z.as_str()).lt_eq(upper_corner.z)),
        )
        .collect()?;
    let filtered_voxel_grid = VoxelGrid::new(
        filtered_voxel_data,
        voxel_grid.info().clone(),
        voxel_grid.reference_frames().clone(),
    )?;

    Ok(filtered_voxel_grid)
}
