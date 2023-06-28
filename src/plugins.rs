use crate::assets::{Aseprite, AsepriteLoader};
use crate::systems::{animate_aseprite, fixup_aseprite_animation, fixup_texture_atlas};
use bevy::prelude::*;

pub struct AsepritePlugin;

impl Plugin for AsepritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Aseprite>()
            .add_asset_loader(AsepriteLoader)
            .add_system(fixup_texture_atlas)
            .add_system(fixup_aseprite_animation)
            .add_system(animate_aseprite.after(fixup_aseprite_animation));
    }
}
