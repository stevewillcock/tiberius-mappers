pub use tiberius_mappers_derive::TryFromRow;

#[cfg(not(any(feature = "tiberius", feature = "tiberius-ng")))]
compile_error!("enable exactly one backend feature: `tiberius` (0.12) or `tiberius-ng` (0.13).");

#[cfg(feature = "tiberius-ng")]
use tiberius_ng as tiberius;

#[doc = include_str!("../README.md")]
#[cfg(all(doctest, feature = "tiberius", not(feature = "tiberius-ng")))]
pub struct ReadmeDocTests;

/// Defines a conversion from a tiberius::Row to a struct.
pub trait TryFromRow {
    /// Try to convert a tiberius::Row to a struct. Returns a Result using the tiberius::error::Error type.
    fn try_from_row(row: tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: Sized;
}
