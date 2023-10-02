# tiberius-mappers

Row mappers for the [Tiberius SQL Server driver](https://github.com/prisma/tiberius).

See the [published crate](https://crates.io/crates/tiberius-mappers) and
the [documentation](https://docs.rs/crate/tiberius-mappers/latest) for more information.

- Allows you to map tiberius rows to structs
- Defines `FromRow` trait for `tiberius::Row`
- Supports deriving the `FromRowOwned` traits for structs via the tiberius-mappers-derive crate
- Handles null values where these map to Option<T> fields in the struct
- Currently maps by name in FromRowBorrowed and by index in FromRowOwned

The existing [tiberius-derive](https://crates.io/crates/tiberius-derive) crate currently offers more options for
mapping, but does not seem to be maintained and doesn't work with newer versions of Tiberius. I have been maintaining a
fork of this crate to support newer versions of Tiberius in internal builds, but I wanted to start from scratch with a
simpler implementation. Note that this implementation is based on the original tiberius-derive crate, so credit to the
original authors for the idea and some of the code.

## Usage

This is a work in progress. Currently, the `TryFromRow` mapper is implemented.

```rust

use tiberius_mappers::TryFromRow;

#[derive(TryFromRow)] // Derive the FromRow trait on our struct
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub description: Option<String>,
}

pub async fn print_customers(rows: Vec<tiberius::Row>) -> Result<(), Box<dyn std::error::Error>> {
    let customers: Vec<Customer> = rows.into_iter().map(Customer::try_from_row).collect::<Result<Vec<Customer>, _>>()?;

    for customer in customers {
        println!("Customer: {} - {:?} - {:?}", customer.id, customer.first_name, customer.last_name);
    }

    Ok(())
}


```

## TODO

- Add more tests (proc macros are not as straightforward to test!)
- Add an option to validate the row names in the returned query result set against the struct field for
  safety
- Improve error messages
- Possibly support renaming fields (maybe, not sure if this is a good idea). This would need to interact with the row
  name validation option mentioned above.