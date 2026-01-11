use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[uniform(101)]
    pub min_z: f32,
    #[uniform(101)]
    pub max_z: f32,
}

impl BackgroundMaterial {
    const FRAGMENT_SHADER_PATH: &str = "shaders/background_space.wgsl";
}

impl Material for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        BackgroundMaterial::FRAGMENT_SHADER_PATH.into()
    }
}
