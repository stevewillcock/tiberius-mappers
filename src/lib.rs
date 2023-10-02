use tiberius::FromSql;
pub use tiberius_mappers_derive::FromRowBorrowed;
pub use tiberius_mappers_derive::FromRowOwned;

pub trait FromRowBorrowed<'a> {
    fn from_row_borrowed(row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: Sized;
}

pub fn map_field<'a, T>(row: &'a tiberius::Row, field_name: &str) -> Result<T, tiberius::error::Error>
where
    T: FromSql<'a>,
{
    row.try_get::<T, &str>(field_name)?
        .ok_or_else(|| tiberius::error::Error::Conversion(format!("None value for non optional field {}", field_name).into()))
}

pub fn map_optional_field<'a, T>(row: &'a tiberius::Row, field_name: &str) -> Result<Option<T>, tiberius::error::Error>
where
    T: FromSql<'a>,
{
    row.try_get::<T, &str>(field_name)
}

pub trait FromRowOwned {
    fn from_row_owned(row: tiberius::Row) -> Result<Self, tiberius::error::Error>
    where
        Self: Sized;
}
