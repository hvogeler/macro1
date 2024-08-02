use darling::{util::PathList, FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Path};

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(custom_model), supports(struct_named))]
struct CustomModelArgs {
    #[darling(default, multiple, rename = "model")]
    pub models: Vec<CustomModel>,
}

#[derive(FromMeta, Clone)]
struct CustomModel {
    name: String,
    fields: PathList,
    #[darling(default)]
    extra_derives: PathList,
}

pub(crate) fn derive_custom_model_impl(input: TokenStream) -> TokenStream {
    let orig_struct = parse_macro_input!(input as DeriveInput);
    let DeriveInput { data, .. } = orig_struct.clone();
    match data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let args = match CustomModelArgs::from_derive_input(&orig_struct) {
                Ok(v) => v,
                Err(e) => {
                    return TokenStream::from(e.write_errors());
                }
            };
            let CustomModelArgs { models } = args;
            let mut implementation = quote!();

            if models.is_empty() {
                panic!("Specify at least one custom model");
            }

            for model in models {
                let generated_model = generate_custom_model(&fields, &model);
                implementation.extend(quote!(#generated_model));
            }

            implementation.into()
        }
        _ => panic!("Not supported"),
    }
}

fn generate_custom_model(fields: &Fields, model: &CustomModel) -> proc_macro2::TokenStream {
    let CustomModel {
        name,
        fields: target_fields,
        extra_derives,
    } = model;

    let mut new_fields = quote!();
    for Field {
        ident,
        attrs,
        vis,
        colon_token,
        ty,
        ..
    } in fields
    {
        println!("Generating field {}", ident.clone().unwrap().to_string());
        let Some(ident) = ident else {
            panic!("failed to get struct identifier");
        };

        let path = match Path::from_string(&ident.clone().to_string()) {
            Ok(path) => path,
            Err(e) => panic!("failed to convert field identifier to path: {e:?}"),
        };

        if !target_fields.contains(&path) {
            continue;
        }

        new_fields.extend(quote! {
            #(#attrs)*
            #vis #ident #colon_token #ty,
        });
    }

    let output_struct_ident = match Ident::from_string(name) {
        Ok(ident) => ident,
        Err(e) => panic!("{e:?}"),
    };

    let mut output_extra_derives = quote!();

    if !extra_derives.is_empty() {
        output_extra_derives.extend(quote! {
            #(#extra_derives,)*
        });
    }

    quote! {
        #[derive(#output_extra_derives)]
        pub struct #output_struct_ident {
            #new_fields
        }
    }
}
