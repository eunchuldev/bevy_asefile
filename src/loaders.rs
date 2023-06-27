/*
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use asefile::AsepriteFile;
use crate::assets::Aseprite;

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
            let aesfile = AsepriteFile::read(bytes)?;
            for frame in 0..asefile.num_frames() {
                let texture = Image::new(
                    Extent3d {
                        width: image.width(),
                        height: image.height(),
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    asefile.frame(frame).image().into_raw(),
                    TextureFormat::Rgba8UnormSrgb,
                );
            }
            let textures: Vec<Image> = (0..asefile.num_frames()).map(|idx| asefile.frame(idx).image()).map(|image| Image::new(
                Extent3d {
                    width: image.width(),
                    height: image.height(),
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                image.into_raw(),
                TextureFormat::Rgba8UnormSrgb,
            )).collect();
            let layer_textures: Vec<Vec<(String, Image)>> = asefile.layers().map(|layer| (layer.name().clone(), (0..asefile.num_frames())
                .map(|idx| layer.frame(idx).image())
                .map(|image| Image::new(
                    Extent3d {
                        width: image.width(),
                        height: image.height(),
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    image.into_raw(),
                    TextureFormat::Rgba8UnormSrgb,
                ))
                .collect()));


            for idx in 0..asefile.num_frames() {
                asefile.frame(idx).image()
            }
            asefile.frames.map(|layer| (layer.name(), layer.frame()));
            /*let aseprite = Aseprite {
                file,
                layer_images,
                images,
            };*/
            load_context.set_default_asset(LoadedAsset::new(aseprite));
            //load_context.set_default_asset(LoadedAsset::new(aseprite));
            /*let mut atlas = TextureAtlasBuilder::default();
            for (idx, image) in aseprite.*/
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}
*/
