use proc_macro::TokenStream;
use syn::Data;
use quote::quote;

pub(crate) fn into_hash_map_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            // let mut implementation = quote! {
            //     let mut hash_map = std::collections::HashMap::<String, String>::new();
            // };

            // for field in fields {
            //     let identifier = field.ident.as_ref().unwrap();
            //     implementation.extend(quote! {
            //         hash_map.insert(
            //             stringify!(#identifier).to_string(),
            //             String::from(value.#identifier));
            //     });
            // }
            let field_idents = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
            quote! {
                #[automatically_derived]
                impl From<#struct_identifier> for std::collections::HashMap<String, String> {
                    fn from(value: #struct_identifier) -> Self {
                        let mut hash_map = std::collections::HashMap::<String, String>::new();
                        println!("impl From<#struct_identifier> for HashMap");
                        #(
                            hash_map.insert(stringify!(#field_idents).to_string(), value.#field_idents.to_string());
                        )*

                        hash_map
                    }
                }
            }
        }
        _ => {
            unimplemented!()
        },
    }.into()
}