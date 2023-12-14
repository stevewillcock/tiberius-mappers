use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident};

fn impl_from_trait_for_row(ast: DeriveInput) -> proc_macro2::TokenStream {
    let ident: Ident = ast.ident;

    let mut field_metadatas: Vec<Field> = vec![];

    match ast.data {
        syn::Data::Struct(data) => {
            for field in data.fields {
                if (field.ident).is_some() {
                    field_metadatas.push(field)
                }
            }
        }
        _ => panic!("Only structs are supported by tiberius mappers derive"),
    };

    let field_mappers: Vec<proc_macro2::TokenStream> = field_metadatas
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            let f_ident = field.ident.unwrap();
            let f_type = field.ty;
            // This is very closely based on code from tiberius_derive
            quote! {
                    #f_ident: {
                        macro_rules! read_data {
                            (Option<$f_type: ty>) => { {
                                    <$f_type as tiberius::FromSqlOwned>::from_sql_owned(
                                        row_iter.next().ok_or_else(|| tiberius::error::Error::Conversion(format!("Could not find field {} from column with index {}", stringify!(#f_ident), #idx).into()))?
                            ).map_err(|e| tiberius::error::Error::Conversion(format!("Could not convert type for optional field {} from column index {} with underlying error {}", stringify!(#f_ident), #idx, e).into()))?
                               } };
                            ($f_type: ty) => { {
                                (<$f_type as tiberius::FromSqlOwned>::from_sql_owned(
                                    row_iter.next().ok_or_else(|| tiberius::error::Error::Conversion(format!("Could not find field {} from column with index {}", stringify!(#f_ident), #idx).into()))?
                                ).map_err(|e| tiberius::error::Error::Conversion(format!("Could not convert type for non optional field {} from column index {} with underlying error {}", stringify!(#f_ident), #idx, e).into()))?
                                ).ok_or_else(|| tiberius::error::Error::Conversion(format!(r"None value for non optional field {} from column with index {}", stringify!(#f_ident), #idx).into()))?
                            }};
                        };

                        read_data!(#f_type)
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl TryFromRow for #ident {

            fn try_from_row(row: tiberius::Row) -> Result<Self, tiberius::error::Error> where Self: Sized {
                let mut row_iter = row.into_iter();
                Ok(Self {
                    #(#field_mappers,)*
                })
            }
        }
    }
}

#[proc_macro_derive(TryFromRow)]
pub fn from_row_derive_macro_owned(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_from_trait_for_row(ast).into()
}
