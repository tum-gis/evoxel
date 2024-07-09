use polars::error::PolarsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EvoxelError(#[from] evoxel_core::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parsing(#[from] serde_json::Error),
    #[error(transparent)]
    Polars(#[from] PolarsError),
    #[error(transparent)]
    Ecoord(#[from] ecoord::io::Error),
}
