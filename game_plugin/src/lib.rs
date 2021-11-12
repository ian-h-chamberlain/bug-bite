#![allow(clippy::too_many_arguments)] // systems quickly gain lots of args
#![allow(clippy::needless_pass_by_value)] // Res<T>
#![allow(clippy::default_trait_access)] // general ..Default::default()
#![allow(clippy::type_complexity)] // for QuerySet<(...)>
#![allow(clippy::module_name_repetitions)] // meh, maybe reconsider this one

mod actions;
mod audio;
mod loading;
mod menu;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::AppBuilder;
use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    Loading,
    /// During this State the actual game logic is executed
    Playing,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
