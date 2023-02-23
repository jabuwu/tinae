use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;
use bevy_spine::{prelude::*, SpineSynchronizerSet, SpineSynchronizerSystem};

use crate::transform2::Transform2;

pub struct SpinePlugin;

impl Plugin for SpinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_spine::SpinePlugin)
            .add_plugin(SpineSync2Plugin::default())
            .add_system(spine_attach_transform2);
    }
}

fn spine_attach_transform2(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
) {
    for event in spine_ready_event.iter() {
        for (_, bone_entity) in event.bones.iter() {
            commands.entity(*bone_entity).insert(Transform2::default());
        }
    }
}

pub struct SpineSynchronizer2Plugin<T: Component> {
    _marker: PhantomData<T>,
}

impl<T: Component> Default for SpineSynchronizer2Plugin<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<T: Component + Clone + Copy + Debug + PartialEq + Eq + Hash> Plugin
    for SpineSynchronizer2Plugin<T>
{
    fn build(&self, app: &mut App) {
        app.add_system(
            spine_sync_entities_2::<T>
                .in_set(SpineSynchronizerSystem::<T>::SyncEntities)
                .after(SpineSystem::Update)
                .after(SpineSynchronizerSet::<T>::BeforeSync)
                .before(SpineSynchronizerSet::<T>::DuringSync),
        )
        .add_system(
            spine_sync_bones_2::<T>
                .in_set(SpineSynchronizerSystem::<T>::SyncBones)
                .after(SpineSynchronizerSystem::<T>::SyncEntities)
                .after(SpineSynchronizerSet::<T>::DuringSync),
        )
        .add_system(
            spine_sync_entities_applied_2::<T>
                .in_set(SpineSynchronizerSystem::<T>::SyncEntitiesApplied)
                .after(SpineSynchronizerSystem::<T>::SyncBones)
                .before(SpineSynchronizerSet::<T>::AfterSync)
                .before(SpineSystem::Render),
        );
    }
}

pub fn spine_sync_entities_2<S: Component + Clone + Copy + Debug + PartialEq + Eq + Hash>(
    mut bone_query: Query<(&mut Transform2, &SpineBone)>,
    spine_query: Query<&Spine, With<S>>,
) {
    for (mut bone_transform, bone) in bone_query.iter_mut() {
        if let Ok(spine) = spine_query.get(bone.spine_entity) {
            if let Some(bone) = bone.handle.get(&spine.skeleton) {
                bone_transform.translation.x = bone.x();
                bone_transform.translation.y = bone.y();
                bone_transform.rotation = bone.rotation().to_radians();
                bone_transform.scale.x = bone.scale_x();
                bone_transform.scale.y = bone.scale_y();
            }
        }
    }
}

pub fn spine_sync_bones_2<S: Component + Clone + Copy + Debug + PartialEq + Eq + Hash>(
    mut bone_query: Query<(&mut Transform2, &SpineBone)>,
    mut spine_query: Query<&mut Spine, With<S>>,
) {
    for (bone_transform, bone) in bone_query.iter_mut() {
        if let Ok(mut spine) = spine_query.get_mut(bone.spine_entity) {
            if let Some(mut bone) = bone.handle.get_mut(&mut spine.skeleton) {
                bone.set_x(bone_transform.translation.x);
                bone.set_y(bone_transform.translation.y);
                bone.set_rotation(bone_transform.rotation.to_degrees());
                bone.set_scale_x(bone_transform.scale.x);
                bone.set_scale_y(bone_transform.scale.y);
            }
        }
    }
    for mut spine in spine_query.iter_mut() {
        spine.0.skeleton.update_world_transform();
    }
}

pub fn spine_sync_entities_applied_2<
    S: Component + Clone + Copy + Debug + PartialEq + Eq + Hash,
>(
    mut bone_query: Query<(&mut Transform2, &SpineBone)>,
    spine_query: Query<&Spine, With<S>>,
) {
    for (mut bone_transform, bone) in bone_query.iter_mut() {
        if let Ok(spine) = spine_query.get(bone.spine_entity) {
            if let Some(bone) = bone.handle.get(&spine.skeleton) {
                bone_transform.translation.x = bone.applied_x();
                bone_transform.translation.y = bone.applied_y();
                bone_transform.rotation = bone.applied_rotation().to_radians();
                bone_transform.scale.x = bone.applied_scale_x();
                bone_transform.scale.y = bone.applied_scale_y();
            }
        }
    }
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Default, Debug, Hash)]
pub struct SpineSync2;

pub type SpineSync2System = SpineSynchronizerSystem<SpineSync2>;
pub type SpineSync2Set = SpineSynchronizerSet<SpineSync2>;
pub type SpineSync2Plugin = SpineSynchronizer2Plugin<SpineSync2>;
