mod game;
mod menu;

use bevy::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, GamePlugin, MenuPlugin))
        .add_systems(Startup, camera)
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

