use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcoordError(#[from] ecoord::Error),
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),

    #[error("No data: {0}")]
    NoData(&'static str),
    #[error("Lengths don't match: {0}")]
    ShapeMisMatch(&'static str),

    #[error("Field {0} does not match type")]
    TypeMisMatch(&'static str),
    #[error("unknown data store error")]
    ColumnNameMisMatch,
}
