use crate::assets::{Aseprite, AsepriteLoader};
use crate::systems::{animate_aseprite, fixup_aseprite_animation, fixup_texture_atlas};
//use crate::ui::systems::{fixup_aseprite_animation_ui, animate_aseprite_ui, fixup_ninepatch_ui};
use bevy::prelude::*;

pub struct AsepritePlugin;

impl Plugin for AsepritePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Aseprite>()
            .add_asset_loader(AsepriteLoader)
            .add_systems(PreUpdate, fixup_texture_atlas)
            .add_systems(PreUpdate, fixup_aseprite_animation)
            .add_systems(Update, animate_aseprite);
        /*
        .add_systems(PreUpdate, fixup_aseprite_animation_ui)
        .add_systems(PreUpdate, fixup_ninepatch_ui)
        .add_systems(Update, animate_aseprite_ui);*/
    }
}
