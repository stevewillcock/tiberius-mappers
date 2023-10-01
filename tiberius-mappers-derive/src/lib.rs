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
        impl<'a> FromRowZeroCopy<'a> for #ident<'a> {
            fn from_row_zero_copy(row: &'a tiberius::Row) -> Result<Self, tiberius::error::Error> where Self: Sized {
                Ok(Self {
                    #(#field_mappers,)*
                })
            }
        }
    })
    .into()
}

#[proc_macro_derive(FromRowZeroCopy)]
pub fn from_row_derive_macro(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_from_row_trait(ast).into()
}
