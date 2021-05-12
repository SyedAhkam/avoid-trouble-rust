use bevy::prelude::*;

mod game;
mod player;
mod obstacle;
mod ui;

use game::Game;

pub const BACKGROUND_COLOR: &str = "2E3440";

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Avoid Trouble (Rust)".to_string(),
            width: 700.,
            height: 500.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::hex(BACKGROUND_COLOR).unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(Game::new())
        .run();
}
