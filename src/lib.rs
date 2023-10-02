pub use tiberius_mappers_derive::TryFromRow;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;

/// Defines a conversion from a tiberius::Row to a struct.
pub trait TryFromRow {
    /// Try to convert a tiberius::Row to a struct. Returns a Result using the tiberius::error::Error type.
    fn try_from_row(row: tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: Sized;
}
