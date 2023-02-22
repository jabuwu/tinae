use std::{
    any::type_name,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::prelude::*;

use crate::transform2::{Depth, Transform2};

pub struct ScreenFadePlugin;

impl Plugin for ScreenFadePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenFade>()
            .add_event::<ScreenFadeOutEvent>()
            .add_startup_system(screen_fade_spawn)
            .add_system(screen_fade_update);
    }
}

#[derive(Default, Resource)]
pub struct ScreenFade {
    disabled: bool,
    fade_out: bool,
    alpha: f32,
    context: Option<ScreenFadeContext>,
}

impl ScreenFade {
    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn fade_in(&mut self) {
        self.fade_out = false;
    }

    pub fn fade_out(&mut self, context: impl Hash) {
        self.context = Some(ScreenFadeContext::new(context));
        self.fade_out = true;
    }

    fn update(&mut self, delta_seconds: f32) {
        if self.fade_out {
            self.alpha += delta_seconds * 3.;
            if self.disabled {
                self.alpha = 1.;
            }
        } else {
            self.alpha -= delta_seconds * 3.;
            if self.disabled {
                self.alpha = 0.;
            }
        }
        self.alpha = self.alpha.clamp(0., 1.);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ScreenFadeContext(u64);

impl ScreenFadeContext {
    pub fn new<T: Hash>(value: T) -> Self {
        let mut hasher = DefaultHasher::new();
        type_name::<T>().hash(&mut hasher);
        value.hash(&mut hasher);
        Self(hasher.finish())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenFadeOutEvent {
    context: ScreenFadeContext,
}

impl ScreenFadeOutEvent {
    pub fn in_context(&self, context: impl Hash) -> bool {
        self.context == ScreenFadeContext::new(context)
    }
}

#[derive(Component)]
pub struct ScreenFadeEntity;

fn screen_fade_spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE),
                color: Color::rgba(0., 0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        },
        Transform2::new().with_scale(Vec2::splat(100000.)),
        Depth::Exact(1.),
        ScreenFadeEntity,
    ));
}

fn screen_fade_update(
    mut screen_fade: ResMut<ScreenFade>,
    mut screen_fade_query: Query<&mut Sprite, With<ScreenFadeEntity>>,
    mut screen_fade_out_events: EventWriter<ScreenFadeOutEvent>,
    time: Res<Time>,
) {
    screen_fade.update(time.delta_seconds());
    if screen_fade.alpha == 1. {
        if let Some(context) = screen_fade.context.take() {
            screen_fade_out_events.send(ScreenFadeOutEvent { context });
        }
    }
    for mut screen_fade_sprite in screen_fade_query.iter_mut() {
        if !screen_fade.disabled {
            screen_fade_sprite
                .color
                .set_a(1. - (1. - screen_fade.alpha).powf(2.));
        } else {
            screen_fade_sprite.color.set_a(0.);
        }
    }
}
