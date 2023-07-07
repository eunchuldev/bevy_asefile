use asefile::Tag;
use asefile::{AsepriteFile, LayerType};
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::GenericImage;
use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, TypeUuid)]
#[uuid = "f10252cf-4b15-43a6-a8b2-811c408b2758"]
pub struct Aseprite {
    pub path: PathBuf,
    pub layers: Vec<String>,
    pub tags: Vec<Tag>,
    pub frame_durations: Vec<Duration>,
    pub num_frames: u32,
    pub atlas_indexes: Vec<u32>,
    pub atlas_layer_indexes: HashMap<(u32, u32), u32>, // (layer, frame)
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
    pub fn frame_duration(&self, frame: usize) -> Duration {
        self.frame_durations[frame]
    }
    pub fn atlas_range(&self, layer_name: Option<&str>, tag_name: Option<&str>) -> Range<u32> {
        let tag = tag_name.map(|name| self.tag(name));
        let layer_id = layer_name.map(|name| self.layer_id(name));
        match (tag, layer_id) {
            (Some(tag), Some(layer_id)) => {
                let start_idx = self.atlas_layer_indexes[&(layer_id, tag.from_frame())];
                let end_idx = self.atlas_layer_indexes[&(layer_id, tag.to_frame())];
                start_idx..(end_idx + 1u32)
            }
            (Some(tag), None) => {
                let start_idx = self.atlas_indexes[tag.from_frame() as usize];
                let end_idx = self.atlas_indexes[tag.to_frame() as usize];
                start_idx..(end_idx + 1u32)
            }
            (None, Some(layer_id)) => {
                let start_idx = self.atlas_layer_indexes[&(layer_id, 0)];
                let end_idx = self.atlas_layer_indexes[&(layer_id, self.num_frames - 1)];
                start_idx..(end_idx + 1u32)
            }
            (None, None) => 0..self.num_frames,
        }
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
            let atlas_layer_indexes = HashMap::from_iter(
                non_empty_cels
                    .iter()
                    .enumerate()
                    .map(|(i, cel)| ((cel.layer(), cel.frame()), i as u32 + num_frames)),
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
                        let index = atlas_layer_indexes[&(parent_cel.layer(), parent_cel.frame())];
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
            let atlas = load_context.set_labeled_asset(
                "atlas",
                LoadedAsset::new(TextureAtlas::from_grid(
                    texture,
                    Vec2::new(cel_width as f32 - padding.x, cel_height as f32 - padding.y),
                    num_rows as usize,
                    num_rows as usize,
                    Some(padding),
                    None,
                )),
            );
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
            let aseprite = Aseprite {
                path: load_context.path().to_path_buf(),
                atlas,
                layers,
                tags,
                num_frames,
                frame_durations: (0..num_frames)
                    .map(|frame| Duration::from_millis(asefile.frame(frame).duration() as u64))
                    .collect(),
                atlas_indexes: (0..num_frames).collect(),
                atlas_layer_indexes,
            };
            load_context.set_default_asset(LoadedAsset::new(aseprite));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}
