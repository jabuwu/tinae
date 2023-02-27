use std::collections::{HashMap, HashSet};

use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
};

/// Adds the [`SubAssets`] resource to the app.
pub struct SubAssetsPlugin;

impl Plugin for SubAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SubAssets>();
    }
}

/// Sub asset tracker for complex asset types. Find child assets for an asset that loads additional
/// assets.
///
/// This class isn't magic. It's a helpful utility but tracking of parent/child assets must be done
/// manually using [`SubAssets::add`] and [`SubAssets::remove`].
#[derive(Resource, Default, Clone, Debug)]
pub struct SubAssets {
    map: HashMap<HandleId, HashSet<HandleId>>,
}

impl SubAssets {
    /// Iterate over all sub assets for the given asset.
    pub fn children(&self, parent: HandleId) -> Vec<HandleId> {
        let mut children = vec![];
        if let Some(set) = self.map.get(&parent) {
            for child in set.iter() {
                children.extend(self.children(*child));
                children.push(*child);
            }
        }
        children
    }

    /// Add a sub asset to the parent asset.
    pub fn add(&mut self, parent: HandleId, child: HandleId) -> bool {
        let set = self.map.entry(parent).or_insert_with(|| HashSet::new());
        set.insert(child)
    }

    /// Remove a sub asset from the parent asset.
    pub fn remove(&mut self, parent: HandleId, child: HandleId) -> bool {
        if let Some(set) = self.map.get_mut(&parent) {
            if set.remove(&child) {
                if set.len() == 0 {
                    self.map.remove(&parent);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Remove the sub asset from any parent asset. More expensive than [`SubAssets::remove`] since
    /// it must iterate over all parents, but useful if the parent asset isn't known.
    pub fn remove_all(&mut self, child: HandleId) -> bool {
        let mut removed = false;
        let parents: Vec<HandleId> = self.map.iter().map(|(parent, _)| *parent).collect();
        for parent in parents.into_iter() {
            if self.remove(parent, child) {
                removed = true;
            }
        }
        removed
    }

    /// Clear all sub assets in a parent.
    pub fn clear(&mut self, parent: HandleId) {
        self.map.remove(&parent);
    }

    /// Get the load state of an asset and its sub assets. Uses
    /// [`AssetServer::get_group_load_state`] internally.
    pub fn get_load_state<H: Into<HandleId>>(
        &self,
        asset_server: &AssetServer,
        handle: H,
    ) -> LoadState {
        let mut all_handles = vec![];
        let handle_id: HandleId = handle.into();
        for child in self.children(handle_id).into_iter() {
            all_handles.push(child);
        }
        all_handles.push(handle_id);
        asset_server.get_group_load_state(all_handles)
    }

    /// The same as [`AssetServer::get_group_load_state`], but also checks sub assets.
    pub fn get_group_load_state(
        &self,
        asset_server: &AssetServer,
        handles: impl IntoIterator<Item = HandleId>,
    ) -> LoadState {
        let mut all_handles = vec![];
        for parent in handles {
            for child in self.children(parent).into_iter() {
                all_handles.push(child);
            }
            all_handles.push(parent);
        }
        asset_server.get_group_load_state(all_handles)
    }
}
