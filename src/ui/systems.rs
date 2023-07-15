use crate::assets::Aseprite;
use crate::components::{AsepriteAnimation, AsepriteAtlas, AsepriteUiBundle, AsepriteUiChildren, AsepriteUiChild, AsepriteUiChildBundle, AsepriteUiNinepatch};
use crate::ui::components::{AsepriteUiBundle, AsepriteUiChildren, AsepriteUiChild, AsepriteUiChildBundle, AsepriteUiNinepatch};
use bevy::prelude::*;
use bevy::ui::widget::UiImageSize;

pub fn fixup_aseprite_animation_ui(
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<
        (
            &Handle<Aseprite>,
            &AsepriteAtlas,
            &mut AsepriteAnimation,
            &mut UiTextureAtlasImage,
        ),
        Changed<AsepriteAtlas>,
    >,
) {
    for (aseprite_handle, ase_atlas, mut ase_anim, mut sprite) in query.iter_mut() {
        if let Some(aseprite) = aseprites.get(aseprite_handle) {
            sprite.index = ase_anim.fixup(ase_atlas, aseprite);
            info!("fixup index: {}", sprite.index);
            info!("fixup atlas: {:?}", ase_atlas);
        } else {
            error!("Fail to load aseprite from handle");
        }
    }
}

pub fn animate_aseprite_ui(
    time: Res<Time>,
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<(
        &Handle<Aseprite>,
        &mut AsepriteAnimation,
        &mut UiTextureAtlasImage,
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

pub fn fixup_ninepatch_ui(
    mut commands: Commands,
    aseprites: Res<Assets<Aseprite>>,
    mut query: Query<(
        Entity,
        &mut AsepriteUiChildren,
        &AsepriteAtlas,
        &Handle<Aseprite>,
    ), (Changed<AsepriteAtlas>, Without<AsepriteUiChild>)>,
    mut children_query: Query<&mut AsepriteAtlas, With<AsepriteUiChild>>,
) {
    for (entity, mut children, atlas, aseprite_handle) in query.iter_mut() {
        *children = match *children {
            AsepriteUiChildren::NoChild => {
                let aseprite = coalesce!(aseprites.get(aseprite_handle), continue);
                if let Some(slice_name) = atlas.slice 
                && let slice = aseprite.slice(slice_name, 0)
                && let Some(ninepatch_center) = slice.ninepatch_center {
                    let mut ninepatches = [entity; 9];
                    for i in 0u8..9 {
                        ninepatches[i as usize] = commands.spawn(AsepriteUiChildBundle {
                            style: Style{
                                position_type: PositionType::Absolute,
                                top: match i / 3 { 0 => Val::Px(0.0), 1 => Val::Px(ninepatch_center.min.y), _ => Val::Auto },
                                bottom: match i / 3 { 0 => Val::Auto, 1 => Val::Px(slice.size.y - ninepatch_center.max.y), _ => Val::Px(0.0) },
                                left: match i % 3 { 0 => Val::Px(0.0), 1 => Val::Px(ninepatch_center.min.x), _ => Val::Auto },
                                right: match i % 3 { 0 => Val::Auto, 1 => Val::Px(slice.size.x - ninepatch_center.max.x), _ => Val::Px(0.0) },
                                height: match i / 3 { 0 => Val::Px(ninepatch_center.min.y), 1 => Val::Auto, _ => Val::Px(slice.size.y - ninepatch_center.max.y) },
                                width: match i % 3 { 0 => Val::Px(ninepatch_center.min.x), 1 => Val::Auto, _ => Val::Px(slice.size.x - ninepatch_center.max.x) },
                                ..Default::default()
                            },
                            z_index: ZIndex::Local(-1),
                            aseprite: aseprite_handle.clone(),
                            aseprite_atlas: AsepriteAtlas { ninepatch: Some(i), ..atlas.clone() },
                            texture_atlas: aseprite.atlas.clone(),
                            ..Default::default()
                        }).id();
                    }
                    commands.entity(entity).push_children(&ninepatches).id();
                    AsepriteUiChildren::Ninepatches(ninepatches)
                } else {
                    let child = commands.spawn(AsepriteUiChildBundle {
                        aseprite: aseprite_handle.clone(),
                        aseprite_atlas: atlas.clone(),
                        texture_atlas: aseprite.atlas.clone(),
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            ..Default::default()
                        },
                        ..Default::default()
                    }).id();
                    info!("spawn sprite");
                    commands.entity(entity).add_child(child);
                    AsepriteUiChildren::Sprite(child)
                }
            }
            AsepriteUiChildren::Sprite(entity) => {
                if let Ok(mut child_atlas) = children_query.get_mut(entity) {
                    if child_atlas.tag != atlas.tag { child_atlas.tag = atlas.tag.clone(); }
                    if child_atlas.slice != atlas.slice { child_atlas.slice = atlas.slice.clone(); }
                    if child_atlas.layer != atlas.layer { child_atlas.layer = atlas.layer.clone(); }
                }
                continue;
            }
            AsepriteUiChildren::Ninepatches(ninepatches) => {
                let mut iter = children_query.iter_many_mut(ninepatches.iter());
                while let Some(mut child_atlas) = iter.fetch_next() {
                    if child_atlas.tag != atlas.tag { child_atlas.tag = atlas.tag.clone(); }
                    if child_atlas.slice != atlas.slice { child_atlas.slice = atlas.slice.clone(); }
                    if child_atlas.layer != atlas.layer { child_atlas.layer = atlas.layer.clone(); }
                }
                continue;
            }
        }
    }
}
