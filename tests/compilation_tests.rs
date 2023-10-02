#[cfg(test)]
mod test {
    use tiberius::error::Error;
    use tiberius::{FromSqlOwned, Row};
    use tiberius_mappers::{FromRowBorrowed, FromRowOwned};

    #[test]
    fn can_compile_manual_borrowed_trait() {
        #[allow(dead_code)]
        struct Test<'a> {
            id: i32,
            name: &'a str,
        }

        impl<'a> FromRowBorrowed<'a> for Test<'a> {
            fn from_row_borrowed(row: &'a Row) -> Result<Self, Error>
            where
                Self: Sized,
            {
                Ok(Self {
                    id: row.try_get::<i32, &str>("id")?.ok_or_else(|| {
                        Error::Conversion(
                            format!("None value for non optional field {}", "id").into(),
                        )
                    })?,
                    name: row.try_get::<&str, &str>("name")?.ok_or_else(|| {
                        Error::Conversion(
                            format!("None value for non optional field {}", "name").into(),
                        )
                    })?,
                })
            }
        }
    }

    #[test]
    fn can_compile_derived_borrowed_trait() {
        #[derive(FromRowBorrowed)]
        #[allow(dead_code)]
        struct Test<'a> {
            id: i32,
            name: &'a str,
        }
    }

    #[test]
    fn can_compile_manual_owned_trait() {
        #[allow(dead_code)]
        struct Test {
            id: i32,
            z_location: i32,
        }

        impl FromRowOwned for Test {
            fn from_row_owned(row: Row) -> Result<Self, Error>
            where
                Self: Sized,
            {
                let mut row_iter = row.into_iter();

                Ok(Self {
                    id: <i32 as FromSqlOwned>::from_sql_owned(row_iter.next().ok_or_else(
                        || {
                            Error::Conversion(
                                format!("Could not find value for field {}", "id").into(),
                            )
                        },
                    )?)?
                    .ok_or_else(|| {
                        Error::Conversion(
                            format!("None value for non optional field {}", "id").into(),
                        )
                    })?,
                    z_location: <i32 as FromSqlOwned>::from_sql_owned(
                        row_iter.next().ok_or_else(|| {
                            Error::Conversion(
                                format!("None value for non optional field {}", "z_location")
                                    .into(),
                            )
                        })?,
                    )?
                    .ok_or_else(|| {
                        Error::Conversion(
                            format!("None value for non optional field {}", "z_location").into(),
                        )
                    })?,
                })
            }
        }
    }

    #[test]
    fn can_compile_derived_owned_trait() {
        #[derive(FromRowOwned)]
        #[allow(dead_code)]
        struct Test {
            id: i32,
            z_location: i32,
        }
    }
}
