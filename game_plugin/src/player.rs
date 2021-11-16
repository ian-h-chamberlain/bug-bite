use crate::actions::{Actions, ViewMode};
use crate::GameState;

use bevy::prelude::*;

mod scope_sprite;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(scope_sprite::spawn_scope.system())
                .with_system(spawn_camera.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(control_crosshair.system()),
        );
    }
}

struct MainCamera;

struct ScopeMask;
struct Crosshair;

fn spawn_camera(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::WHITE;

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn control_crosshair(
    windows: Res<Windows>,
    actions: Res<Actions>,
    mut q: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<&mut Visible, With<ScopeMask>>,
        Query<&mut Transform, With<Crosshair>>,
    )>,
) {
    // UNWRAP: There is always exactly one primary window
    let window = windows.get_primary().unwrap();

    // UNWRAP: There is exactly one crosshairs
    // check if the cursor is in the primary window
    if let Some(pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        let q_camera = q.q0();
        // UNWRAP: There is exactly one MainCamera entity
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        let mut crosshair_transform = q.q2_mut().single_mut().unwrap();
        crosshair_transform.translation = world_pos.truncate();
    }

    for mut visible in q.q1_mut().iter_mut() {
        visible.is_visible = match actions.view_mode {
            ViewMode::Sights => true,
            ViewMode::Spotting => false,
        };
    }
}
