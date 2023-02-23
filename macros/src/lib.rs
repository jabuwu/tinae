use proc_macro::TokenStream;

#[proc_macro_derive(AssetStruct, attributes(asset))]
pub fn derive_asset_struct(input: TokenStream) -> TokenStream {
    asset_struct::derive_asset_struct(input)
}

mod asset_struct;
