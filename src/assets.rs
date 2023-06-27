use bevy::{
    reflect::TypeUuid,
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use asefile::AsepriteFile;
use image::GenericImage;
pub use asefile::{Tag, AnimationDirection};

#[derive(Debug, TypeUuid)]
#[uuid = "f10252cf-4b15-43a6-a8b2-811c408b2758"]
pub struct Aseprite {
    pub layers: Vec<String>,
    pub tags: Vec<Tag>,
    pub atlas: Handle<TextureAtlas>,
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

            let num_frames = asefile.num_frames();
            let num_layers = asefile.num_layers();
            let width = asefile.width() as u32;
            let height = asefile.width() as u32;
            let mut buffer = image::RgbaImage::new(width * num_frames, height * (num_layers + 1));

            for frame in 0..num_frames {
                for layer in 0..num_layers {
                    buffer.copy_from(&asefile.cel(frame, layer).image(), width * frame, height * layer)?;
                }
                buffer.copy_from(&asefile.frame(frame).image(), width * frame, height * num_layers)?;
            }
            let texture = load_context.set_labeled_asset("texture", LoadedAsset::new(Image::new(
                Extent3d { width: buffer.width(), height: buffer.height(), depth_or_array_layers: 1, },
                TextureDimension::D2,
                buffer.into_raw(),
                TextureFormat::Rgba8UnormSrgb,
            )));
            let atlas = load_context.set_labeled_asset("atlas", LoadedAsset::new(TextureAtlas::from_grid(
                texture,
                Vec2::new(width as f32, height as f32), 
                num_frames as usize, 
                num_layers as usize + 1,
                None, None,
            )));
            let layers = asefile.layers().map(|layer| layer.name().to_owned()).collect();
            let tags = (0..asefile.num_tags()).map(|i| asefile.tag(i).clone()).collect();
            let aseprite = Aseprite {
                atlas,
                layers,
                tags,
            };
            load_context.set_default_asset(LoadedAsset::new(aseprite));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ase", "aseprite"]
    }
}
