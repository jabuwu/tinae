use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[derive(Clone)]
pub struct WorldDependencies {
    dependency_map: HashMap<String, WorldDependency>,
    span: Span,
    index: usize,
}

impl WorldDependencies {
    pub fn new() -> Self {
        Self {
            dependency_map: HashMap::new(),
            span: Span::call_site(),
            index: 0,
        }
    }
}

#[derive(Clone)]
struct WorldDependency {
    ident: Ident,
    resource: TokenStream,
    mutable: bool,
}

impl WorldDependencies {
    fn depend_on_internal(&mut self, tokens: TokenStream, mutable: bool) -> Ident {
        let tokens_string = tokens.to_string();
        if let Some(mut world_dependency) = self.dependency_map.get_mut(&tokens_string) {
            if mutable {
                world_dependency.mutable = true;
            }
            world_dependency.ident.clone()
        } else {
            let ident = Ident::new(&format!("world_dep_{}", self.index), self.span);
            self.dependency_map.insert(
                tokens_string,
                WorldDependency {
                    ident: ident.clone(),
                    resource: tokens,
                    mutable,
                },
            );
            self.index += 1;
            ident
        }
    }

    pub fn depend_on(&mut self, tokens: TokenStream) -> Ident {
        self.depend_on_internal(tokens, false)
    }

    pub fn depend_on_mut(&mut self, tokens: TokenStream) -> Ident {
        self.depend_on_internal(tokens, true)
    }

    pub fn tokens(self, world: Ident) -> TokenStream {
        let mut dependencies: Vec<WorldDependency> =
            self.dependency_map.into_iter().map(|(_, b)| b).collect();
        if dependencies.is_empty() {
            return quote! {};
        }
        dependencies.sort_by(|a, b| a.ident.to_string().cmp(&b.ident.to_string()));
        let idents: Vec<TokenStream> = dependencies
            .iter()
            .map(|world_dependency| {
                let ident = world_dependency.ident.clone();
                let mutable = if world_dependency.mutable {
                    quote! { mut }
                } else {
                    quote! {}
                };
                quote! { #mutable #ident }
            })
            .collect();
        let resources: Vec<TokenStream> = dependencies
            .iter()
            .map(|world_dependency| world_dependency.resource.clone())
            .collect();
        quote! {
            let mut system_state: bevy::ecs::system::SystemState<(
                #(#resources,)*
            )> = bevy::ecs::system::SystemState::new(#world);
            let (#(#idents,)*) = system_state.get_mut(#world);
        }
    }
}

#[cfg(test)]
mod test {
    use proc_macro2::Span;
    use quote::quote;

    use super::*;

    #[test]
    fn world_dependencies() {
        let mut world_dependencies = WorldDependencies::new();

        assert_eq!(
            world_dependencies
                .clone()
                .tokens(Ident::new("my_world", Span::call_site()))
                .to_string(),
            quote! {}.to_string()
        );

        let world_dep_0 = world_dependencies.depend_on(quote! { Res<MyResource> });
        assert_eq!(world_dep_0.to_string(), "world_dep_0");

        let tokens = world_dependencies
            .clone()
            .tokens(Ident::new("my_world", Span::call_site()));
        assert_eq!(
            tokens.to_string(),
            quote! {
                let mut system_state: bevy::ecs::system::SystemState<(
                    Res<MyResource>,
                )> = bevy::ecs::system::SystemState::new(my_world);
                let (world_dep_0,) = system_state.get_mut(my_world);
            }
            .to_string()
        );

        let world_dep_1 = world_dependencies.depend_on_mut(quote! { Res<MyOtherResource> });
        assert_eq!(world_dep_1.to_string(), "world_dep_1");

        let world_dep_0_again = world_dependencies.depend_on_mut(quote! { Res<MyResource> });
        assert_eq!(world_dep_0_again.to_string(), "world_dep_0");

        let tokens = world_dependencies.tokens(Ident::new("world", Span::call_site()));
        assert_eq!(
            tokens.to_string(),
            quote! {
                let mut system_state: bevy::ecs::system::SystemState<(
                    Res<MyResource>,
                    Res<MyOtherResource>,
                )> = bevy::ecs::system::SystemState::new(world);
                let (mut world_dep_0, mut world_dep_1,) = system_state.get_mut(world);
            }
            .to_string()
        );
    }
}
