use crate::assets::Aseprite;
use bevy::prelude::*;
use bevy::ui::{ContentSize, FocusPolicy, widget::UiImageSize};
use std::ops::Range;
use std::time::Duration;
use crate::components::{AsepriteAnimation, AsepriteAtlas, AsepriteUiBundle, AsepriteUiChildren, AsepriteUiChild, AsepriteUiChildBundle, AsepriteUiNinepatch};



#[derive(Component, Default)]
pub enum AsepriteUiChildren {
    #[default]
    NoChild,
    Sprite(Entity),
    Ninepatches([Entity; 9]),
}

#[derive(Component, Default)]
pub struct AsepriteUiChild;


#[derive(Component, Default, Deref, Clone, Copy)]
pub struct AsepriteUiNinepatch(pub u8);

#[derive(Bundle, Default)]
pub struct AsepriteUiChildBundle {
    pub aseprite: Handle<Aseprite>,
    pub aseprite_atlas: AsepriteAtlas,
    pub aseprite_animation: AsepriteAnimation,

    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,

    pub marker: AsepriteUiChild,

    pub calculated_size: ContentSize,
    pub texture_atlas: Handle<TextureAtlas>,
    pub texture_atlas_image: UiTextureAtlasImage,
    pub image_size: UiImageSize,
    pub border_color: BorderColor,
}

#[derive(Bundle, Default)]
pub struct AsepriteUiBundle {
    pub aseprite: Handle<Aseprite>,
    pub aseprite_atlas: AsepriteAtlas,
    pub aseprite_animation: AsepriteAnimation,

    pub node: Node,
    pub style: Style,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub z_index: ZIndex,

    pub children: AsepriteUiChildren,
}
