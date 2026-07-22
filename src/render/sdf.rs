mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::{Handle, uuid_handle},
    ecs::{component::Component, resource::Resource},
    math::Vec3,
    render::{
        render_resource::{ShaderType, UniformBuffer},
        texture::CachedTexture,
    },
    shader::Shader,
};

pub use pipeline::SdfPipeline;
pub use prepare::prepare_occluder_meta;
pub use prepare::prepare_sdf_texture;

pub const SDF_SHADER: Handle<Shader> = uuid_handle!("16251728-6dd9-481e-95a7-7c2e0ff8d920");

pub use node::sdf;

#[derive(Component)]
pub struct SdfTexture {
    pub sdf: CachedTexture,
}
#[derive(Resource, Default)]
pub struct OccluderMetaBuffer {
    pub buffer: UniformBuffer<OccluderMeta>,
}

#[derive(Default, ShaderType)]
pub struct OccluderMeta {
    pub count: u32,
    // WebGL2 structs must be 16 byte aligned.
    _padding: Vec3,
}

impl OccluderMeta {
    pub fn new(count: u32) -> Self {
        Self {
            count,
            _padding: Vec3::ZERO,
        }
    }
}
