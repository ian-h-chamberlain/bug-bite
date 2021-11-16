use bevy::prelude::*;

use crate::loading::TextureAssets;
use crate::GameState;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_character));
    }
}

fn spawn_character(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.stickman.clone().into()),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 10.0),
            scale: Vec3::splat(0.08),
            ..Default::default()
        },
        ..Default::default()
    });
}
