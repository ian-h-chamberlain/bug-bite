use crate::GameState;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(set_actions)
                // TODO: kinda hacky, pause menu would be better
                .with_system(exit_on_esc_system),
        );
    }
}

#[derive(Debug)]
pub enum ViewMode {
    Sights,
    Spotting,
}

impl ViewMode {
    fn toggle(&mut self) {
        *self = match self {
            Self::Sights => Self::Spotting,
            Self::Spotting => Self::Sights,
        };
    }
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::Spotting
    }
}

#[derive(Default)]
pub struct Actions {
    pub view_mode: ViewMode,
}

// TODO: https://bevy-cheatbook.github.io/cookbook/mouse-grab.html
//
fn set_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    // TODO: would just_released be better here?
    if keyboard_input.just_pressed(KeyCode::Space) {
        actions.view_mode.toggle();
    }
}
