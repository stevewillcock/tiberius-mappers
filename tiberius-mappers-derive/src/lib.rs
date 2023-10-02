use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident, Path};

fn type_is_option(typ: &syn::Type) -> bool {
    let option = syn::parse_str::<Path>("Option").unwrap();

    match typ {
        syn::Type::Path(typepath) if typepath.qself.is_none() => typepath.path.segments.iter().any(|s| s.ident == option.segments[0].ident),
        _ => false,
    }
}

fn impl_from_row_trait(ast: DeriveInput) -> proc_macro2::TokenStream {
    let ident: Ident = ast.ident;

    let mut field_metadatas: Vec<Field> = vec![];

    match ast.data {
        syn::Data::Struct(data) => {
            for field in data.fields {
                if (&field.ident).is_some() {
                    field_metadatas.push(field)
                }
            }
        }
        _ => panic!("Only structs are supported by tiberius mappers derive"),
    };

    let field_mappers: Vec<proc_macro2::TokenStream> = field_metadatas
        .iter()
        .map(|f| {
            let f_ident = f.ident.as_ref().unwrap();
            let f_ident_str = f_ident.to_string();

            if type_is_option(&f.ty) {
                quote! {
                    #f_ident: tiberius_mappers::map_optional_field(row, #f_ident_str)?
                }
            } else {
                quote! {
                    #f_ident: tiberius_mappers::map_field(row, #f_ident_str)?
                }
            }
        })
        .collect();

    (quote! {
        impl<'a> FromRowBorrowed<'a> for #ident<'a> {
            fn from_row_borrowed(row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error> where Self: Sized {
                Ok(Self {
                    #(#field_mappers,)*
                })
            }
        }
    })
    .into()
}

fn impl_from_row_trait_owned(ast: DeriveInput) -> proc_macro2::TokenStream {
    let ident: Ident = ast.ident;

    let mut field_metadatas: Vec<Field> = vec![];

    match ast.data {
        syn::Data::Struct(data) => {
            for field in data.fields {
                if (&field.ident).is_some() {
                    field_metadatas.push(field)
                }
            }
        }
        _ => panic!("Only structs are supported by tiberius mappers derive"),
    };

    let owned_field_mappers: Vec<proc_macro2::TokenStream> = field_metadatas
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            let f_ident = field.ident.unwrap();
            let f_type = field.ty;
            // println!("*** f_ident: {:?}", f_ident);
            quote! {
                    #f_ident: {
                        macro_rules! read_owned_data {
                            (Option<$f_type: ty>) => { {
                                    // println!("*** Option1 {}", stringify!(#f_ident));
                                    <$f_type as tiberius::FromSqlOwned>::from_sql_owned(row_iter.next().ok_or_else(
                                        || tiberius::error::Error::Conversion(
                                            format!("Could not find field {} from column with index {}", stringify!(#f_ident), #idx).into()
                                        )
                                    )?)?
                               } };
                            ($f_type: ty) => { {
                                // println!("*** Option2 {}", stringify!(#f_ident));
                                (<$f_type as tiberius::FromSqlOwned>::from_sql_owned(row_iter.next().ok_or_else(
                                    || tiberius::error::Error::Conversion(
                                        format!("Could not find field {} from column with index {}", stringify!(#f_ident), #idx).into()
                                    )
                                )?)?).ok_or_else(
                                    || tiberius::error::Error::Conversion(
                                        format!(r" None value for non optional field {} from column with index {}", stringify!(#f_ident), #idx).into()
                                    )
                                )?
                            }};
                        };

                        read_owned_data!(#f_type)
                }
            }
        })
        .collect::<Vec<_>>();

    (quote! {
        impl FromRowOwned for #ident {
            fn from_row_owned(row: tiberius::Row) -> Result<Self, tiberius::error::Error> where Self: Sized {
                let mut row_iter = row.into_iter();
                Ok(Self {
                    #(#owned_field_mappers,)*
                })
            }
        }
    })
    .into()
}

#[proc_macro_derive(FromRowBorrowed)]
pub fn from_row_derive_macro(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_from_row_trait(ast).into()
}

#[proc_macro_derive(FromRowOwned)]
pub fn from_row_derive_macro_owned(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_from_row_trait_owned(ast).into()
}
