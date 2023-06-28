use crate::assets::Aseprite;
use bevy::prelude::*;
use std::ops::Range;
use std::time::Duration;

#[derive(Component, Default)]
pub struct AsepriteAtlas {
    pub layer: Option<&'static str>,
    pub tag: Option<&'static str>,
    //pub slice: Option<&'static str>,
}

#[derive(Default)]
pub enum AnimationDirection {
    #[default]
    Forward,
    Backward,
    PingPong,
}

impl From<asefile::AnimationDirection> for AnimationDirection {
    fn from(ase_anim_dir: asefile::AnimationDirection) -> Self {
        match ase_anim_dir {
            asefile::AnimationDirection::Forward => AnimationDirection::Forward,
            asefile::AnimationDirection::Reverse => AnimationDirection::Backward,
            asefile::AnimationDirection::PingPong => AnimationDirection::PingPong,
        }
    }
}

#[derive(Component, Default)]
pub struct AsepriteAnimation {
    pub frame_rate_multiplier: f32,
    pub direction: AnimationDirection,
    pub time_elapsed: Duration,
    pub current_index: u32,
    pub index_range: Range<u32>,
    pub pong: bool,
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
