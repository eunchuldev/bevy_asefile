use crate::assets::Aseprite;
use crate::components::{AsepriteAtlas, AsepriteAnimation};
use bevy::prelude::*;

pub fn fixup_texture_atlas(
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<(
        &Handle<Aseprite>,
        &mut Handle<TextureAtlas>,
    ), (
        Changed<AsepriteAtlas>,
    )>,
) {
    for (aseprite_handle, mut texture_atlas) in query.iter_mut() {
        if let Some(aseprite) = aseprites.get(aseprite_handle) {
            *texture_atlas = aseprite.atlas.clone();
            info!("fixup texture atlas");
        } else {
           error!("Fail to load aseprite from handle");
        }
    }
}


/*
pub fn sync_up_aseprite_atlas_and_sprite_sheet(
    mut query: Query<(
        &Handle<Aseprite>,
        &AsepriteAtlas,
        &mut TextureAtlasSprite,
    ), (
        Changed<AsepriteAtlas>,
    )>,
) {
    for (aseprite, aseprite_atlas, mut texture_atlas, mut sprite) in query.iter_mut() {
    }
}
*/

/*

pub fn update_animations(
    time: Res<Time>,
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<(
        &Handle<Aseprite>,
        &AsepriteAtlas,
        &mut AsepriteAnimation,
        &mut TextureAtlasSprite,
    )>,
) {
    for (aseprite, atlas, mut animation, mut sprite) in query.iter_mut() {

    if self.tag_changed {
        self.reset(info);
        return true;
    }

    if self.is_paused() {
        return false;
    }

    self.time_elapsed += dt;
    let mut current_frame_duration = self.current_frame_duration(info);
    let mut frame_changed = false;
    while self.time_elapsed >= current_frame_duration {
        self.time_elapsed -= current_frame_duration;
        self.next_frame(info);
        current_frame_duration = self.current_frame_duration(info);
        frame_changed = true;
    }
    frame_changed
}
    
*/
