#[cfg(test)]
mod test {
    use tiberius::error::Error;
    use tiberius::{FromSqlOwned, Row};
    use tiberius_mappers::TryFromRow;

    #[test]
    fn can_compile_manual_trait_impl() {
        #[allow(dead_code)]
        struct Test {
            id: i32,
            z_location: i32,
        }
        impl TryFromRow for Test {
            fn try_from_row(row: Row) -> Result<Test, Error>
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
        #[derive(TryFromRow)]
        #[allow(dead_code)]
        struct Test {
            id: i32,
            z_location: i32,
        }
    }
}
