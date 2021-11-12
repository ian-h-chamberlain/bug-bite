use bevy::prelude::*;
use bevy::render::pipeline::{
    BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite, PipelineDescriptor,
    RenderPipeline,
};
use bevy::render::render_graph::{base, AssetRenderResourcesNode, RenderGraph};
use bevy::render::texture::TextureFormat;
use bevy::sprite::build_sprite_pipeline;

use crate::loading::TextureAssets;

pub(super) fn spawn_scope(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    windows: Res<Windows>,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    const SCOPE_MAT: &str = "scope_material";

    // Ref https://github.com/fahyc/Censorship/tree/master/Assets/Art/FogOfWar
    // since I did basically this once before, lol

    // TODO: Unclear if my usage or just a bevy bug, but frequently crashes in
    /*
            4: bevy_wgpu::renderer::wgpu_render_resource_context::WgpuRenderResourceContext::copy_buffer_to_buffer
                      at /Users/ianchamberlain/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_wgpu-0.5.0/src/renderer/wgpu_render_resource_context.rs:58:27
            5: <bevy_wgpu::renderer::wgpu_render_context::WgpuRenderContext as bevy_render::renderer::render_context::RenderContext>::copy_buffer_to_buffer
                      at /Users/ianchamberlain/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_wgpu-0.5.0/src/renderer/wgpu_render_context.rs:86:9
    */
    let pipeline = pipelines.add(PipelineDescriptor {
        color_target_states: vec![ColorTargetState {
            format: TextureFormat::default(),
            color_blend: BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendState {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Min,
            },
            write_mask: ColorWrite::ALL,
        }],
        // Reuse the regular sprite pipeline, shaders, etc
        ..build_sprite_pipeline(&mut shaders)
    });

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to
    // our shader
    render_graph.add_system_node(
        SCOPE_MAT,
        AssetRenderResourcesNode::<ColorMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new material node to the main pass node. This
    // ensures `SCOPE_MAT` runs before the main pass
    render_graph
        .add_node_edge(SCOPE_MAT, base::node::MAIN_PASS)
        .unwrap();

    let window = windows.get_primary().unwrap();
    let screen_size = Vec2::new(window.width() as f32, window.height() as f32);

    // Spawn a big black rectangle
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(ColorMaterial::color(Color::BLACK)),
        sprite: Sprite::new(screen_size),
        mesh: meshes.add(shape::Quad::new(screen_size).into()),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    });

    // and the
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(textures.crosshair.clone().into()),
            visible: Visible {
                is_visible: false,
                is_transparent: false,
            },
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline {
                pipeline,
                ..Default::default()
            }]),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                scale: Vec3::splat(0.15),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(super::Crosshair);
}
