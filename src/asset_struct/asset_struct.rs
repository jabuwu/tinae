use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::{
    asset::{HandleId, LoadState},
    ecs::system::{Resource, SystemState},
    prelude::*,
};
pub use tinae_macros::AssetStruct;

use crate::fixed_timestep::AddFixedEvent;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AssetStructSystem {
    Update,
    Events,
}

pub trait AddAssetStruct {
    fn add_asset_struct<T: Default + AssetStruct>(&mut self) -> &mut Self;
}

impl AddAssetStruct for App {
    fn add_asset_struct<T: Default + AssetStruct>(&mut self) -> &mut Self {
        self.init_resource::<T>()
            .insert_resource(AssetStructState::<T> {
                load: false,
                unload: false,
                loading: false,
                status: AssetStructStatus::NotLoaded,
                frame_delay: AssetStructFrameDelay::new(2),
                _marker: PhantomData,
            })
            .add_fixed_event::<AssetStructLoadedEvent<T>>()
            .add_fixed_event::<AssetStructFailedEvent<T>>()
            .add_fixed_event::<AssetStructLoadEvent<T>>()
            .add_fixed_event::<AssetStructUnloadEvent<T>>()
            .add_system(
                asset_struct_update::<T>
                    .in_set(AssetStructSystem::Update)
                    .in_base_set(CoreSet::PostUpdate),
            )
            .add_system(
                asset_struct_events::<T>
                    .in_set(AssetStructSystem::Events)
                    .in_base_set(CoreSet::PreUpdate),
            );
        self
    }
}

pub trait AssetStruct: Resource {
    fn load(&mut self, world: &mut World);
    fn unload(&mut self);

    /// Returns all handles in the asset struct, plus all sub assets that were loaded as a result
    /// of loading the assets.
    fn handles(&self, world: &mut World) -> Vec<HandleId>;

    fn status(&self, world: &mut World) -> AssetStructStatus {
        let handles = self.handles(world);
        let mut system_state: SystemState<Res<AssetServer>> = SystemState::new(world);
        let asset_server = system_state.get_mut(world);
        let total = handles.len();
        let mut loaded = 0;
        let mut loading = false;
        for handle in handles.into_iter() {
            match asset_server.get_load_state(handle) {
                LoadState::Loaded => {
                    loaded += 1;
                    loading = true;
                }
                LoadState::Failed => {
                    return AssetStructStatus::Failed;
                }
                LoadState::NotLoaded => {}
                LoadState::Loading => {}
                LoadState::Unloaded => {}
            }
        }
        if !loading {
            AssetStructStatus::NotLoaded
        } else if loaded == total {
            AssetStructStatus::Loaded
        } else {
            AssetStructStatus::Loading {
                progress: loaded as f32 / total as f32,
            }
        }
    }

    /// Returns a hash of all handle IDs. Useful to check if the list of assets has changed because
    /// new sub assets were found.
    fn handles_hash(&self, world: &mut World) -> u64 {
        let mut hasher = DefaultHasher::new();
        for handle in self.handles(world).into_iter() {
            match handle {
                HandleId::Id(_, id) => id.hash(&mut hasher),
                HandleId::AssetPathId(path_id) => path_id.label_id().hash(&mut hasher),
            }
        }
        hasher.finish()
    }
}

#[derive(Default)]
pub struct AssetStructLoadEvent<T: AssetStruct> {
    _marker: PhantomData<T>,
}

#[derive(Default)]
pub struct AssetStructUnloadEvent<T: AssetStruct> {
    _marker: PhantomData<T>,
}

pub struct AssetStructLoadedEvent<T: AssetStruct> {
    _marker: PhantomData<T>,
}

pub struct AssetStructFailedEvent<T: AssetStruct> {
    _marker: PhantomData<T>,
}

#[derive(Resource)]
pub struct AssetStructState<T: AssetStruct> {
    load: bool,
    unload: bool,
    loading: bool,
    status: AssetStructStatus,
    frame_delay: AssetStructFrameDelay,
    _marker: PhantomData<T>,
}

/// Responsible for delaying loading completion for N frames if new sub assets are found. Sub
/// assets may not be found right away, but should be within 1 frame.
struct AssetStructFrameDelay {
    delayed_frames: u64,
    counted_frames: u64,
    last_hash: u64,
}

impl AssetStructFrameDelay {
    fn new(delayed_frames: u64) -> Self {
        Self {
            delayed_frames,
            counted_frames: 0,
            last_hash: 0,
        }
    }

    fn can_complete_loading(&mut self, handles_hash: u64) -> bool {
        self.counted_frames += 1;
        if handles_hash != self.last_hash {
            self.last_hash = handles_hash;
            self.counted_frames = 0;
        }
        self.counted_frames > self.delayed_frames
    }
}

impl<T: AssetStruct> AssetStructState<T> {
    pub fn status(&self) -> AssetStructStatus {
        self.status
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum AssetStructStatus {
    #[default]
    NotLoaded,
    Loading {
        progress: f32,
    },
    Loaded,
    Failed,
}

fn asset_struct_update<T: AssetStruct>(world: &mut World) {
    world.resource_scope(|world, mut state: Mut<AssetStructState<T>>| {
        if state.unload {
            world.resource_scope(|_, mut asset_struct: Mut<T>| {
                asset_struct.unload();
                state.loading = false;
            });
        } else if state.load {
            if !state.loading {
                world.resource_scope(|world, mut asset_struct: Mut<T>| {
                    asset_struct.load(world);
                    state.loading = true;
                });
            }
        }
        state.load = false;
        state.unload = false;
        if state.loading {
            let (handles_hash, status) = world.resource_scope(|world, asset_struct: Mut<T>| {
                (asset_struct.handles_hash(world), asset_struct.status(world))
            });
            let mut system_state: SystemState<(
                EventWriter<AssetStructLoadedEvent<T>>,
                EventWriter<AssetStructFailedEvent<T>>,
            )> = SystemState::new(world);
            let (mut asset_struct_loaded_events, mut asset_struct_failed_events) =
                system_state.get_mut(world);
            if state.frame_delay.can_complete_loading(handles_hash) {
                state.status = status;
                match state.status {
                    AssetStructStatus::Loaded => {
                        asset_struct_loaded_events.send(AssetStructLoadedEvent::<T> {
                            _marker: PhantomData,
                        });
                        state.loading = false;
                    }
                    AssetStructStatus::Failed => {
                        asset_struct_failed_events.send(AssetStructFailedEvent::<T> {
                            _marker: PhantomData,
                        });
                        state.loading = false;
                    }
                    _ => {}
                }
            }
        }
    });
}

fn asset_struct_events<T: AssetStruct>(
    mut asset_struct_load_events: EventReader<AssetStructLoadEvent<T>>,
    mut asset_struct_unload_events: EventReader<AssetStructUnloadEvent<T>>,
    mut asset_struct_state: ResMut<AssetStructState<T>>,
) {
    let mut load = false;
    let mut unload = false;
    for _ in asset_struct_load_events.iter() {
        load = true;
    }
    for _ in asset_struct_unload_events.iter() {
        unload = true;
    }
    if load {
        asset_struct_state.load = true;
    }
    if unload {
        asset_struct_state.unload = true;
    }
}
