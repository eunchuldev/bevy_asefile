use crate::assets::Aseprite;
use bevy::prelude::*;
use std::ops::Range;
use std::time::Duration;

#[derive(Component, Clone, Default, Eq, PartialEq, Debug)]
pub struct AsepriteAtlas {
    pub layer: Option<&'static str>,
    pub tag: Option<&'static str>,
    pub slice: Option<&'static str>,
    pub ninepatch: Option<u8>,
}

#[derive(Default, Clone, Debug)]
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

#[derive(Component, Clone, Default, Debug)]
pub struct AsepriteAnimation {
    pub frame_rate_multiplier: f32,
    pub direction: AnimationDirection,
    pub time_elapsed: Duration,
    pub current_index: u32,
    pub index_range: Range<u32>,
    pub pong: bool,
}

impl AsepriteAnimation {
    pub fn fixup(&mut self, ase_atlas: &AsepriteAtlas, aseprite: &Aseprite) -> usize {
        let atlas_range = aseprite.atlas_range(
            ase_atlas.layer,
            ase_atlas.tag,
            ase_atlas.slice,
            ase_atlas.ninepatch,
        );
        self.direction = ase_atlas
            .tag
            .map(|name| aseprite.tag(name).animation_direction().into())
            .unwrap_or_default();
        self.current_index = match self.direction {
            AnimationDirection::Forward | AnimationDirection::PingPong => atlas_range.start,
            AnimationDirection::Backward => atlas_range.end - 1,
        };
        self.index_range = atlas_range;
        self.time_elapsed = Duration::from_millis(0);
        self.current_index as usize
    }

    pub fn step(&mut self, elapsed: Duration, aseprite: &Aseprite) -> usize {
        let mut current_index = self.current_index;
        let mut current_frame_duration =
            aseprite.frame_duration((self.current_index - self.index_range.start) as usize);
        while self.time_elapsed >= current_frame_duration {
            self.time_elapsed -= current_frame_duration;
            current_index = match self.direction {
                AnimationDirection::Forward => {
                    if current_index + 1 >= self.index_range.end {
                        self.index_range.start
                    } else {
                        current_index + 1
                    }
                }
                AnimationDirection::Backward => {
                    if current_index - 1 < self.index_range.start {
                        self.index_range.end - 1
                    } else {
                        current_index - 1
                    }
                }
                AnimationDirection::PingPong => {
                    if !self.pong {
                        if current_index + 1 >= self.index_range.end {
                            self.pong = true;
                            current_index - 1
                        } else {
                            current_index + 1
                        }
                    } else if current_index - 1 < self.index_range.start {
                        self.pong = false;
                        current_index + 1
                    } else {
                        current_index - 1
                    }
                }
            };
            current_frame_duration =
                aseprite.frame_duration((current_index - self.index_range.start) as usize);
        }

        self.current_index = current_index;
        self.time_elapsed += elapsed;

        current_index as usize
    }
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

#[derive(Component, Default)]
pub struct AsepriteNinepatch;
