use bevy::prelude::*;
use crate::assets::Aseprite;
use std::time::Duration;

/*
#[derive(Component)]
pub struct AsepriteTag(&'static str)
#[derive(Component)]
pub struct AsepriteLayer(&'static str)
#[derive(Component)]
pub struct AsepriteSlice(&'static str)
*/

#[derive(Component, Default)]
pub struct AsepriteAtlas {
    pub layer: Option<&'static str>,
    pub slice: Option<&'static str>,
    pub tag: Option<&'static str>,
}

#[derive(Component, Default)]
pub struct AsepriteAnimation {
    pub frame_rate_multiplier: f32,
    pub time_elapsed: Duration,
    pub index: usize,
}

#[derive(Bundle, Default)]
pub struct AsepriteBundle {
    pub aseprite: Handle<Aseprite>,
    pub aseprite_atlas: AsepriteAtlas,
    pub aseprite_animation: AsepriteAnimation,
    
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
