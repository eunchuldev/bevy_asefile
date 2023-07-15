use asefile::Tag;
use asefile::{AsepriteFile, LayerType};
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::GenericImage;
use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Hash, PartialEq, Eq, Debug, Default)]
pub struct AtlasKey {
    layer: Option<u32>,
    slice: Option<u32>,
    frame: u32,
    ninepatch: Option<u8>,
}

#[derive(Debug, Default)]
pub struct SliceSegment {
    pub from_frame: usize,
    pub size: Vec2,
    pub ninepatch_center: Option<Rect>,
}

#[derive(Debug, Default)]
pub struct Slice {
    pub name: String,
    pub segments: Vec<SliceSegment>,
}

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "f10252cf-4b15-43a6-a8b2-811c408b2758"]
pub struct Aseprite {
    pub path: PathBuf,
    pub layers: Vec<String>,
    pub tags: Vec<Tag>,
    pub slices: Vec<Slice>,
    pub frame_durations: Vec<Duration>,
    pub num_frames: u32,
    pub atlas_indexes: HashMap<AtlasKey, u32>,
    pub atlas: Handle<TextureAtlas>,
}

impl Aseprite {
    pub fn layer_id(&self, name: &str) -> u32 {
        self.layers
            .iter()
            .position(|layer| layer == name)
            .unwrap_or_else(|| {
                panic!(
                    "Layer `{}` is not exists at `{}`",
                    name,
                    self.path.display()
                )
            }) as u32
    }
    pub fn tag(&self, name: &str) -> &Tag {
        self.tags
            .iter()
            .find(|tag| tag.name() == name)
            .unwrap_or_else(|| panic!("Tag `{}` is not exists at `{}`", name, self.path.display()))
    }
    pub fn slice(&self, name: &str, frame: usize) -> &SliceSegment {
        let slice = self
            .slices
            .iter()
            .find(|slice| slice.name == name)
            .unwrap_or_else(|| {
                panic!(
                    "Slice `{}` is not exists at `{}`",
                    name,
                    self.path.display()
                )
            });
        &slice.segments[slice
            .segments
            .iter()
            .position(|key| key.from_frame > frame)
            .unwrap_or(slice.segments.len())
            - 1]
    }
    pub fn slice_id(&self, name: &str) -> u32 {
        self.slices
            .iter()
            .enumerate()
            .find(|(_, slice)| slice.name == name)
            .map(|(i, _)| i as u32)
            .unwrap_or_else(|| {
                panic!(
                    "Slice `{}` is not exists at `{}`",
                    name,
                    self.path.display()
                )
            })
    }
    pub fn frame_duration(&self, frame: usize) -> Duration {
        self.frame_durations[frame]
    }
    pub fn atlas_range(
        &self,
        layer_name: Option<&str>,
        tag_name: Option<&str>,
        slice_name: Option<&str>,
        ninepatch: Option<u8>,
    ) -> Range<u32> {
        let tag = tag_name.map(|name| self.tag(name));
        let layer = layer_name.map(|name| self.layer_id(name));
        let slice = slice_name.map(|name| self.slice_id(name));
        if ninepatch.is_some() && slice.is_none() {
            error!("The slice has to be specified when ninepatch is specified");
        }
        if slice_name.is_some() && layer_name.is_some() {
            error!("Both slice and layer specification are not supoported yet");
        }
        /*if slice_name.is_some() && ninepatch.is_some() && self.slice(slice_name.unwrap(), 0).ninepatch_center.is_none() {
            error!("slice `{:?}` has not ninepatch information", slice_name);
        }*/
        let (start_frame, end_frame) = match tag {
            Some(tag) => (tag.from_frame(), tag.to_frame()),
            None => (0, self.num_frames - 1),
        };
        self.atlas_indexes[&AtlasKey {
            layer,
            slice,
            frame: start_frame,
            ninepatch,
        }]..(self.atlas_indexes[&AtlasKey {
            layer,
            slice,
            frame: end_frame,
            ninepatch,
        }] + 1u32)
    }
}

#[derive(Debug, Default)]
pub struct AsepriteLoader;

impl AssetLoader for AsepriteLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            debug!("Loading aseprite at {:?}", load_context.path());
            let asefile = AsepriteFile::read(bytes)?;

            let padding = Vec2::new(1.0, 1.0);
            let num_frames = asefile.num_frames();
            let num_layers = asefile.num_layers();
            let cel_width = asefile.width() as u32 + (padding.x as u32);
            let cel_height = asefile.width() as u32 + (padding.y as u32);
            let non_empty_cels: Vec<_> = (0..num_frames * num_layers)
                .filter(|i| {
                    matches!(asefile.layer(i / num_frames).layer_type(), LayerType::Group)
                        || !asefile.cel(i % num_frames, i / num_frames).is_empty()
                })
                .map(|i| asefile.cel(i % num_frames, i / num_frames))
                .collect();
            let non_empty_cel_images: Vec<_> = non_empty_cels
                .iter()
                .map(|cel| {
                    if !cel.is_empty() {
                        Some(cel.image())
                    } else {
                        None
                    }
                })
                .collect();
            let mut atlas_indexes = HashMap::from_iter(
                (0..num_frames)
                    .map(|i| {
                        (
                            AtlasKey {
                                layer: None,
                                frame: i,
                                slice: None,
                                ninepatch: None,
                            },
                            i,
                        )
                    })
                    .chain(non_empty_cels.iter().enumerate().map(|(i, cel)| {
                        (
                            AtlasKey {
                                layer: Some(cel.layer()),
                                frame: cel.frame(),
                                slice: None,
                                ninepatch: None,
                            },
                            i as u32 + num_frames,
                        )
                    })),
            );

            let num_rows = ((non_empty_cels.len() as u32 + num_frames) as f32).sqrt() as u32 + 1;
            let mut buffer = image::RgbaImage::new(num_rows * cel_width, num_rows * cel_height);

            for index in 0u32..num_frames {
                buffer.copy_from(
                    &asefile.frame(index).image(),
                    index % num_rows * cel_width,
                    index / num_rows * cel_height,
                )?;
            }
            for (i, cel) in non_empty_cels.iter().enumerate() {
                if let Some(image) = &non_empty_cel_images[i] {
                    let index = i as u32 + num_frames;
                    buffer.copy_from(
                        image,
                        index % num_rows * cel_width,
                        index / num_rows * cel_height,
                    )?;

                    let mut parent = asefile.layer(cel.layer()).parent().map(|l| l.id());
                    while let Some(parent_layer) = parent {
                        let parent_cel = asefile.cel(cel.frame(), parent_layer);
                        let index = atlas_indexes[&AtlasKey {
                            layer: Some(parent_cel.layer()),
                            frame: parent_cel.frame(),
                            slice: None,
                            ninepatch: None,
                        }];
                        image::imageops::overlay(
                            &mut buffer,
                            image,
                            index % num_rows * cel_width,
                            index / num_rows * cel_height,
                        );
                        parent = asefile.layer(parent_layer).parent().map(|l| l.id());
                    }
                }
            }

            let texture = load_context.set_labeled_asset(
                "texture",
                LoadedAsset::new(Image::new(
                    Extent3d {
                        width: buffer.width(),
                        height: buffer.height(),
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    buffer.into_raw(),
                    TextureFormat::Rgba8UnormSrgb,
                )),
            );
            let mut atlas = TextureAtlas::from_grid(
                texture,
                Vec2::new(cel_width as f32 - padding.x, cel_height as f32 - padding.y),
                num_rows as usize,
                num_rows as usize,
                Some(padding),
                None,
            );
            for slice in 0u32..(asefile.slices().len() as u32) {
                let keys = &asefile.slices()[slice as usize].keys;
                if keys.is_empty() {
                    continue;
                }
                for frame in 0u32..num_frames {
                    let key_index = keys
                        .iter()
                        .position(|key| key.from_frame > frame)
                        .unwrap_or(keys.len())
                        - 1;
                    let key = &keys[key_index];
                    let rect = atlas.textures[atlas_indexes[&AtlasKey {
                        layer: None,
                        frame,
                        slice: None,
                        ninepatch: None,
                    }] as usize];
                    let slice_rect = Rect::new(
                        rect.min.x + key.origin.0 as f32,
                        rect.min.y + key.origin.1 as f32,
                        rect.min.x + key.origin.0 as f32 + key.size.0 as f32,
                        rect.min.y + key.origin.1 as f32 + key.size.1 as f32,
                    );
                    let index = atlas.add_texture(slice_rect);
                    atlas_indexes.insert(
                        AtlasKey {
                            layer: None,
                            frame,
                            slice: Some(slice),
                            ninepatch: None,
                        },
                        index as u32,
                    );
                }
                if let Some(slice9) = &keys[0].slice9 {
                    for ninepatch in 0u8..9 {
                        for frame in 0u32..num_frames {
                            let rect = atlas.textures[atlas_indexes[&AtlasKey {
                                layer: None,
                                frame,
                                slice: Some(slice),
                                ninepatch: None,
                            }] as usize];
                            let (x1, x2) = match ninepatch % 3 {
                                0 => (rect.min.x, rect.min.x + slice9.center_x as f32),
                                1 => (
                                    rect.min.x + slice9.center_x as f32,
                                    rect.min.x
                                        + slice9.center_x as f32
                                        + slice9.center_width as f32,
                                ),
                                2 => (
                                    rect.min.x
                                        + slice9.center_x as f32
                                        + slice9.center_width as f32,
                                    rect.max.x,
                                ),
                                _ => unreachable!(),
                            };
                            let (y1, y2) = match ninepatch / 3 {
                                0 => (rect.min.y, rect.min.y + slice9.center_y as f32),
                                1 => (
                                    rect.min.y + slice9.center_y as f32,
                                    rect.min.y
                                        + slice9.center_y as f32
                                        + slice9.center_height as f32,
                                ),
                                2 => (
                                    rect.min.y
                                        + slice9.center_y as f32
                                        + slice9.center_height as f32,
                                    rect.max.y,
                                ),
                                _ => unreachable!(),
                            };
                            let slice_rect = Rect::new(x1, y1, x2, y2);
                            let index = atlas.add_texture(slice_rect);
                            atlas_indexes.insert(
                                AtlasKey {
                                    layer: None,
                                    frame,
                                    slice: Some(slice),
                                    ninepatch: Some(ninepatch),
                                },
                                index as u32,
                            );
                        }
                    }
                }
            }

            let atlas = load_context.set_labeled_asset("atlas", LoadedAsset::new(atlas));
            let mut layers: Vec<_> = asefile
                .layers()
                .map(|layer| layer.name().to_owned())
                .collect();
            for layer in asefile.layers() {
                if let Some(parent) = layer.parent() {
                    layers[layer.id() as usize] =
                        layers[parent.id() as usize].clone() + "::" + &layers[layer.id() as usize];
                }
            }
            let tags = (0..asefile.num_tags())
                .map(|i| asefile.tag(i).clone())
                .collect();
            let slices: Vec<_> = asefile
                .slices()
                .iter()
                .map(|slice| Slice {
                    name: slice.name.clone(),
                    segments: slice
                        .keys
                        .iter()
                        .map(|key| SliceSegment {
                            from_frame: key.from_frame as usize,
                            size: Vec2::new(key.size.0 as f32, key.size.1 as f32),
                            ninepatch_center: key.slice9.as_ref().map(|slice9| {
                                Rect::new(
                                    slice9.center_x as f32,
                                    slice9.center_y as f32,
                                    slice9.center_x as f32 + slice9.center_width as f32,
                                    slice9.center_y as f32 + slice9.center_height as f32,
                                )
                            }),
                        })
                        .collect(),
                })
                .collect();

            let aseprite = Aseprite {
                path: load_context.path().to_path_buf(),
                atlas,
                layers,
                tags,
                slices,
                num_frames,
                frame_durations: (0..num_frames)
                    .map(|frame| Duration::from_millis(asefile.frame(frame).duration() as u64))
                    .collect(),
                atlas_indexes,
            };
            load_context.set_default_asset(LoadedAsset::new(aseprite));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}
