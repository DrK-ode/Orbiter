use bevy::{
    prelude::*,
    render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer},
    shader::ShaderRef,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarryMaterial {}

impl StarryMaterial {
    const FRAGMENT_SHADER_PATH: &str = "shaders/star.wgsl";
}

impl Material for StarryMaterial {
    fn fragment_shader() -> ShaderRef {
        StarryMaterial::FRAGMENT_SHADER_PATH.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NoisyMaterial {
    #[storage(100, read_only)]
    pub noise_data: Handle<ShaderStorageBuffer>,
    #[uniform(101)]
    pub noise_width:      u32,
    #[uniform(102)]
    pub noise_height:     u32,
    #[uniform(103)]
    pub star_color: LinearRgba,
}

impl NoisyMaterial {
    const FRAGMENT_SHADER_PATH: &str = "shaders/noisy.wgsl";
}

impl Material for NoisyMaterial {
    fn fragment_shader() -> ShaderRef {
        NoisyMaterial::FRAGMENT_SHADER_PATH.into()
    }
}
