use custom_model::derive_custom_model_impl;
use into_hash_map::into_hash_map_impl;
use proc_macro::TokenStream;

mod into_hash_map;
mod custom_model;

extern crate proc_macro2;

#[proc_macro_derive(IntoHashMap)]
pub fn into_hash_map(item: TokenStream) -> TokenStream {
    into_hash_map_impl(item)
}

#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(item: TokenStream) -> TokenStream {
    derive_custom_model_impl(item)
}