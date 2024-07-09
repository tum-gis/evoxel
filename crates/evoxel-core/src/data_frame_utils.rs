use crate::Error::{ColumnNameMisMatch, NoData, TypeMisMatch};
use crate::{Error, VoxelDataColumnNames, VoxelGridInfo};
use ecoord::ReferenceFrames;
use polars::datatypes::DataType;
use polars::frame::DataFrame;

pub fn check_data_integrity(
    voxel_data: &DataFrame,
    _info: &VoxelGridInfo,
    _reference_frames: &ReferenceFrames,
) -> Result<(), Error> {
    if voxel_data.is_empty() {
        return Err(NoData("voxel_data"));
    }

    let column_names = voxel_data.get_column_names();
    if column_names[0] != VoxelDataColumnNames::X.as_str() {
        return Err(ColumnNameMisMatch);
    }
    if column_names[1] != VoxelDataColumnNames::Y.as_str() {
        return Err(ColumnNameMisMatch);
    }
    if column_names[2] != VoxelDataColumnNames::Z.as_str() {
        return Err(ColumnNameMisMatch);
    }

    let data_types = voxel_data.dtypes();
    if data_types[0] != DataType::Int64 {
        return Err(TypeMisMatch("x"));
    }
    if data_types[1] != DataType::Int64 {
        return Err(TypeMisMatch("y"));
    }
    if data_types[2] != DataType::Int64 {
        return Err(TypeMisMatch("z"));
    }

    Ok(())
}
