use bevy::prelude::*;
use bevy_asefile::{AsepriteAtlas, AsepriteBundle, AsepritePlugin, AsepriteUiBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AsepritePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 10.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(AsepriteUiBundle {
                    aseprite: asset_server.load("ui.aseprite"),
                    aseprite_atlas: AsepriteAtlas {
                        slice: Some("SpeechBubble"),
                        ..Default::default()
                    },
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            "hello",
                            TextStyle {
                                font: asset_server.load("Ramche.ttf"),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..Default::default()
                    });
                });

            parent.spawn(AsepriteUiBundle {
                aseprite: asset_server.load("ui.aseprite"),
                aseprite_atlas: AsepriteAtlas {
                    slice: Some("ThoughtBubble"),
                    ..Default::default()
                },
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
