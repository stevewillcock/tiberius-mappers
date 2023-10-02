pub use tiberius_mappers_derive::TryFromRow;

pub trait TryFromRow {
    fn try_from_row(row: tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: Sized;
}
