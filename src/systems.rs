use crate::assets::Aseprite;
use crate::components::{AsepriteAnimation, AsepriteAtlas};
use crate::utils::coalesce;
use bevy::prelude::*;
use std::ops::DerefMut;

// TODO: use AssetChanged query condition after https://github.com/bevyengine/bevy/pull/5080 merged
pub fn fixup_texture_atlas(
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<(
        &Handle<Aseprite>,
        &mut Handle<TextureAtlas>,
        &mut AsepriteAtlas,
    )>,
    mut ev_asset: EventReader<AssetEvent<Aseprite>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                for (aseprite_handle, mut texture_atlas, mut aseprite_atlas) in query.iter_mut() {
                    if aseprite_handle.id() == handle.id() {
                        if let Some(aseprite) = aseprites.get(aseprite_handle) {
                            *texture_atlas = aseprite.atlas.clone();
                            aseprite_atlas.deref_mut();
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
        let aseprite = coalesce!(aseprites.get(aseprite_handle), continue);
        sprite.index = ase_anim.fixup(ase_atlas, aseprite);
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
            let next_index = ase_anim.step(time.delta(), aseprite);
            if sprite.index != next_index {
                sprite.index = next_index;
            }
        }
    }
}
