use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parenthesized, parse_macro_input, token, Data, DeriveInput, LitStr, Result};

use crate::world_dependencies::WorldDependencies;

struct AssetAttribute {
    _paren_token: token::Paren,
    literal: LitStr,
}

impl Parse for AssetAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(AssetAttribute {
            _paren_token: parenthesized!(content in input),
            literal: content.parse()?,
        })
    }
}

struct SpineAssetAttribute {
    _paren_token: token::Paren,
    skeleton_literal: LitStr,
    _comma_token: Comma,
    atlas_literal: LitStr,
}

impl Parse for SpineAssetAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(SpineAssetAttribute {
            _paren_token: parenthesized!(content in input),
            skeleton_literal: content.parse()?,
            _comma_token: content.parse()?,
            atlas_literal: content.parse()?,
        })
    }
}

pub fn derive_asset_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut load_quotes = vec![];
    let mut status_quotes = vec![];
    let mut load_dependencies = WorldDependencies::new();
    let mut status_dependencies = WorldDependencies::new();
    match input.data {
        Data::Struct(asset_struct) => {
            for field in asset_struct.fields.iter() {
                let field_ident = field.ident.clone().unwrap();
                let mut asset_attribute = None;
                let mut spine_asset_attribute = None;
                let mut found_asset = false;
                for attr in field.attrs.iter() {
                    let Some(attr_path) = attr.path.get_ident().map(|ident| ident.to_string()) else { continue };
                    match attr_path.as_str() {
                        "asset" => {
                            if found_asset {
                                panic!("multiple asset attributes for {}", field_ident.to_string());
                            }
                            found_asset = true;
                            let tokens: TokenStream = attr.tokens.clone().into();
                            asset_attribute = Some(parse_macro_input!(tokens as AssetAttribute));
                        }
                        "spine_asset" => {
                            if found_asset {
                                panic!("multiple asset attributes for {}", field_ident.to_string());
                            }
                            found_asset = true;
                            let tokens: TokenStream = attr.tokens.clone().into();
                            spine_asset_attribute =
                                Some(parse_macro_input!(tokens as SpineAssetAttribute));
                        }
                        _ => unreachable!(),
                    }
                }
                if let Some(asset_attribute) = asset_attribute {
                    // load()
                    {
                        let path = asset_attribute.literal.value();
                        let asset_server = load_dependencies.depend_on(quote! { Res<AssetServer> });
                        load_quotes.push(quote! {
                            self.#field_ident = #asset_server.load(#path);
                        });
                    }

                    // status()
                    {
                        let sub_assets = status_dependencies
                            .depend_on(quote! { Res<tinae::sub_assets::SubAssets>});
                        status_quotes.push(quote! {
                            handles.insert(self.#field_ident.id());
                            for child in #sub_assets.children(self.#field_ident.id()).iter() {
                                handles.insert(*child);
                            }
                        });
                    }
                } else if let Some(spine_asset_attribute) = spine_asset_attribute {
                    // load()
                    {
                        let skeleton = spine_asset_attribute.skeleton_literal.value();
                        let atlas = spine_asset_attribute.atlas_literal.value();
                        let asset_server = load_dependencies.depend_on(quote! { Res<AssetServer> });
                        let skeletons = load_dependencies
                            .depend_on_mut(quote! { ResMut<Assets<bevy_spine::SkeletonData>> });
                        if skeleton.ends_with(".json") {
                            load_quotes.push(quote! {
                                self.#field_ident = #skeletons.add(bevy_spine::SkeletonData::new_from_json(
                                    #asset_server.load(#skeleton),
                                    #asset_server.load(#atlas),
                                ));
                            });
                        } else {
                            load_quotes.push(quote! {
                                self.#field_ident = #skeletons.add(bevy_spine::SkeletonData::new_from_json(
                                    #asset_server.load(#skeleton),
                                    #asset_server.load(#atlas),
                                ));
                            });
                        }
                    }

                    // status()
                    {
                        let sub_assets = status_dependencies
                            .depend_on(quote! { Res<tinae::sub_assets::SubAssets>});
                        status_quotes.push(quote! {
                            for child in #sub_assets.children(self.#field_ident.id()).iter() {
                                handles.insert(*child);
                            }
                        });
                    }
                }
            }
        }
        _ => {}
    }
    let status_asset_server =
        status_dependencies.depend_on(quote! { Res<bevy::asset::AssetServer>});
    let load_dependencies = load_dependencies.tokens(Ident::new("world", Span::call_site()));
    let status_dependencies = status_dependencies.tokens(Ident::new("world", Span::call_site()));
    let expanded = quote! {
        impl tinae::asset_struct::AssetStruct for #name {
            fn load(&mut self, world: &mut bevy::ecs::world::World) {
                #load_dependencies
                #(#load_quotes)*
            }
            fn status(&mut self, world: &mut bevy::ecs::world::World) -> tinae::asset_struct::AssetStructStatus {
                #status_dependencies
                let mut handles: std::collections::HashSet<bevy::asset::HandleId> = std::collections::HashSet::new();
                #(#status_quotes)*
                let total = handles.len();
                let mut loaded = 0;
                let mut loading = false;
                for handle in handles.into_iter() {
                    match #status_asset_server.get_load_state(handle) {
                        bevy::asset::LoadState::Loaded => { loaded += 1; loading = true; }
                        bevy::asset::LoadState::Failed => {
                            return tinae::asset_struct::AssetStructStatus::Failed;
                        }
                        bevy::asset::LoadState::NotLoaded => {}
                        bevy::asset::LoadState::Loading => {}
                        bevy::asset::LoadState::Unloaded => {}
                    }
                }
                if !loading {
                    tinae::asset_struct::AssetStructStatus::NotLoaded
                } else if loaded == total {
                    tinae::asset_struct::AssetStructStatus::Loaded
                } else {
                    tinae::asset_struct::AssetStructStatus::Loading { progress: loaded as f32 / total as f32 }
                }
            }
        }
    };
    TokenStream::from(expanded)
}
