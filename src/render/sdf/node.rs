use bevy::prelude::*;
use bevy::render::renderer::{RenderContext, ViewQuery};

use bevy::render::render_resource::{
    BindGroupEntries, GpuArrayBuffer, Operations, PipelineCache, RenderPassColorAttachment,
    RenderPassDescriptor,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::{ViewUniformOffset, ViewUniforms};
use smallvec::{SmallVec, smallvec};

use crate::render::empty_buffer::EmptyBuffer;
use crate::render::extract::ExtractedLightOccluder2d;

use super::pipeline::SdfPipeline;
use super::{OccluderMetaBuffer, SdfTexture};

const SDF_PASS: &str = "sdf_pass";
const SDF_BIND_GROUP: &str = "sdf_bind_group";

pub fn sdf(
    mut render_context: RenderContext,
    view: ViewQuery<(&ViewUniformOffset, &SdfTexture)>,
    render_device: Res<RenderDevice>,
    sdf_pipeline: Res<SdfPipeline>,
    pipeline_cache: Res<PipelineCache>,
    view_uniforms: Res<ViewUniforms>,
    elo: Res<GpuArrayBuffer<ExtractedLightOccluder2d>>,
    empty: Res<EmptyBuffer>,
    omb: Res<OccluderMetaBuffer>,
) -> Result<(), BevyError> {
    let (view_offset, sdf_texture) = view.into_inner();

    let (
        Some(pipeline),
        Some(view_uniform_binding),
        Some(light_occluders_binding),
        Some(occluder_meta_buffer),
    ) = (
        pipeline_cache.get_render_pipeline(sdf_pipeline.pipeline_id),
        view_uniforms.uniforms.binding(),
        elo.binding().or(empty.binding()),
        omb.buffer.binding(),
    )
    else {
        return Ok(());
    };

    let bind_group = render_context.render_device().create_bind_group(
        SDF_BIND_GROUP,
        &pipeline_cache.get_bind_group_layout(&sdf_pipeline.layout_descriptor),
        &BindGroupEntries::sequential((
            view_uniform_binding.clone(),
            light_occluders_binding,
            occluder_meta_buffer,
        )),
    );

    let mut sdf_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
        label: Some(SDF_PASS),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: &sdf_texture.sdf.default_view,
            resolve_target: None,
            ops: Operations::default(),
            depth_slice: None,
        })],
        ..default()
    });

    let mut dynamic_offsets: SmallVec<[u32; 3]> = smallvec![view_offset.offset];

    // Storage buffers aren't available in WebGL2. We fall back to a
    // dynamic uniform buffer, and therefore need to provide the offset.
    // We're providing a value of 0 here as we're limiting the number of
    // point lights to only those that can reasonably fit in a single binding.
    if render_device.limits().max_storage_buffers_per_shader_stage == 0 {
        dynamic_offsets.push(0);
    }

    sdf_pass.set_render_pipeline(pipeline);
    sdf_pass.set_bind_group(0, &bind_group, &dynamic_offsets);
    sdf_pass.draw(0..3, 0..1);

    Ok(())
}
