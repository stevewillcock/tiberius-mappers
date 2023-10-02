# tiberius-mappers

Row mappers for the [Tiberius SQL Server driver](https://github.com/prisma/tiberius).

- Allows you to map tiberius rows to structs
- Defines `FromRow` trait for `tiberius::Row`
- Supports deriving the `FromRowOwned` traits for structs via the tiberius-mappers-derive crate
- Handles null values where these map to Option<T> fields in the struct
- Currently maps by name in FromRowBorrowed and by index in FromRowOwned

The existing [tiberius-derive](https://crates.io/crates/tiberius-derive) crate currently offers more options for
mapping, but does not seem to be maintained and doesn't work with newer versions of Tiberius. I have been maintaining a
fork of this crate to support newer versions of Tiberius in internal builds, but I wanted to start from scratch with a
simpler implementation.

## Usage

This is a work in progress. Currently, the `FromRowBorrowed` and `FromRowOwned` mapper is implemented.

```rust

#[derive(FromRow)] // Derive the FromRow trait on our struct
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub description: Option<String>,
}

pub async fn print_customers(pool: &Pool<ConnectionManager>) -> Result<(), Box<dyn error::Error>> {
    const SQL: &str = "select top 10 id, first_name, last_name, description from customers;";
    let mut conn = pool.get().await?;
    let rows = conn.query(SQL, &[]).await?.into_first_result().await?;
    // Now we can call the from_row method on each row
    let customers: Vec<Customer> = rows.iter().map(Customer::from_row).collect::<Result<Vec<Customer>, _>>()?;

    for customer in customers {
        println!("Customer: {} - {:?} - {:?}", customer.customer_code, customer.description, customer.dispatch_loc_id);
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
  name validation option mentioned above)