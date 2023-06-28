use crate::assets::Aseprite;
use crate::components::{AnimationDirection, AsepriteAnimation, AsepriteAtlas};
use bevy::prelude::*;
use std::time::Duration;

// TODO: use AssetChanged query condition after https://github.com/bevyengine/bevy/pull/5080 merged
pub fn fixup_texture_atlas(
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<(&Handle<Aseprite>, &mut Handle<TextureAtlas>)>,
    mut ev_asset: EventReader<AssetEvent<Aseprite>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                for (aseprite_handle, mut texture_atlas) in query.iter_mut() {
                    if aseprite_handle.id() == handle.id() {
                        if let Some(aseprite) = aseprites.get(aseprite_handle) {
                            *texture_atlas = aseprite.atlas.clone();
                        } else {
                            error!("Fail to load aseprite from handle");
                        }
                    }
                }
            }
            AssetEvent::Removed { .. } => {}
        }
    }
}

pub fn fixup_aseprite_animation(
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<
        (
            &Handle<Aseprite>,
            &AsepriteAtlas,
            &mut AsepriteAnimation,
            &mut TextureAtlasSprite,
        ),
        Changed<AsepriteAtlas>,
    >,
) {
    for (aseprite_handle, ase_atlas, mut ase_anim, mut sprite) in query.iter_mut() {
        if let Some(aseprite) = aseprites.get(aseprite_handle) {
            let atlas_range = aseprite.atlas_range(ase_atlas.layer, ase_atlas.tag);
            ase_anim.direction = ase_atlas
                .tag
                .map(|name| aseprite.tag(name).animation_direction().into())
                .unwrap_or_default();
            ase_anim.current_index = match ase_anim.direction {
                AnimationDirection::Forward | AnimationDirection::PingPong => atlas_range.start,
                AnimationDirection::Backward => atlas_range.end - 1,
            };
            ase_anim.index_range = atlas_range;
            ase_anim.time_elapsed = Duration::from_millis(0);
            sprite.index = ase_anim.current_index as usize;
        } else {
            error!("Fail to load aseprite from handle");
        }
    }
}

pub fn animate_aseprite(
    time: Res<Time>,
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<(
        &Handle<Aseprite>,
        &mut AsepriteAnimation,
        &mut TextureAtlasSprite,
    )>,
) {
    for (aseprite_handle, mut ase_anim, mut sprite) in query.iter_mut() {
        if let Some(aseprite) = aseprites.get(aseprite_handle) {
            let mut current_index = ase_anim.current_index;
            let mut current_frame_duration = aseprite
                .frame_duration((ase_anim.current_index - ase_anim.index_range.start) as usize);
            while ase_anim.time_elapsed >= current_frame_duration {
                ase_anim.time_elapsed -= current_frame_duration;
                current_index = match ase_anim.direction {
                    AnimationDirection::Forward => {
                        if current_index + 1 >= ase_anim.index_range.end {
                            ase_anim.index_range.start
                        } else {
                            current_index + 1
                        }
                    }
                    AnimationDirection::Backward => {
                        if current_index - 1 < ase_anim.index_range.start {
                            ase_anim.index_range.end - 1
                        } else {
                            current_index - 1
                        }
                    }
                    AnimationDirection::PingPong => {
                        if !ase_anim.pong {
                            if current_index + 1 >= ase_anim.index_range.end {
                                ase_anim.pong = true;
                                current_index - 1
                            } else {
                                current_index + 1
                            }
                        } else if current_index - 1 < ase_anim.index_range.start {
                            ase_anim.pong = false;
                            current_index + 1
                        } else {
                            current_index - 1
                        }
                    }
                };
                current_frame_duration =
                    aseprite.frame_duration((current_index - ase_anim.index_range.start) as usize);
            }
            ase_anim.current_index = current_index;
            ase_anim.time_elapsed += time.delta();
            sprite.index = current_index as usize;
        }
    }
}
