mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::{Handle, uuid_handle},
    ecs::component::Component,
    prelude::Shader,
    render::render_resource::{CachedRenderPipelineId, TextureFormat},
};

pub use pipeline::*;
pub use prepare::*;
pub use node::lighting;

pub const LIGHTING_SHADER: Handle<Shader> = uuid_handle!("22ed6ffe-b47d-4b88-b986-5b0e87b3a240");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightingPipelineKey {
    pub target_format: TextureFormat,
}

#[derive(Component)]
pub struct LightingPipelineId(pub CachedRenderPipelineId);
