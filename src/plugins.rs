use bevy::prelude::*;
use crate::assets::{Aseprite, AsepriteLoader};
use crate::systems::{fixup_texture_atlas};

pub struct AsepritePlugin;

impl Plugin for AsepritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Aseprite>()
            .add_asset_loader(AsepriteLoader)
            .add_system(fixup_texture_atlas);
    }
}
