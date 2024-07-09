use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcoordError(#[from] ecoord::Error),
    #[error(transparent)]
    EvoxelError(#[from] evoxel_core::Error),
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),

    #[error("the data for key `{0}` is not available")]
    LowerCornerMustBeBelowUpperCorner(&'static str),
}
