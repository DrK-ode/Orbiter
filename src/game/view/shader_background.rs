use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d}};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarryMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl StarryMaterial {
    const FRAGMENT_SHADER_PATH: &str = "shaders/star.wgsl";
}

impl Material2d for StarryMaterial {
    fn fragment_shader() -> ShaderRef {
        StarryMaterial::FRAGMENT_SHADER_PATH.into()
    }
    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        AlphaMode2d::Mask(0.5)
    }
}